use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use authorization::ApiKey;
use rocket::{
    form::Form,
    tokio::{self, task},
};

mod authorization;
mod constant;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        rocket::routes![upload_passed_website_to_create_canister_and_return_created_url],
    )
}

#[rocket::post("/upload?<starting_cycles>", data = "<zip_file>")]
async fn upload_passed_website_to_create_canister_and_return_created_url(
    mut zip_file: Form<rocket::fs::TempFile<'_>>,
    starting_cycles: Option<u128>,
    _key: ApiKey<'_>,
) -> Result<String, ()> {
    // * make folder called temp in the current folder
    let mut temp_folder = env::current_dir().unwrap_or_default();
    temp_folder.push("temp");
    if !temp_folder.exists() {
        tokio::fs::create_dir_all(&temp_folder)
            .await
            .unwrap_or_default();
    }

    let uploaded_file_unique_name = format!(
        "{}_{}",
        // * system time to nanosecond string
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
        zip_file.name().unwrap_or("unknown")
    );

    let path_to_save_uploaded_file_to = format!(
        "{}/{}.zip",
        temp_folder.to_str().unwrap_or("/tmp"),
        uploaded_file_unique_name
    );
    zip_file
        .persist_to(&path_to_save_uploaded_file_to)
        .await
        .unwrap();

    // * unzip the file to the temp folder with the same name as the zip file
    let mut unzip_folder = temp_folder.clone();
    unzip_folder.push(uploaded_file_unique_name);
    let mut assets_folder = unzip_folder.clone();
    assets_folder.push("www");
    if !assets_folder.exists() {
        tokio::fs::create_dir_all(&assets_folder)
            .await
            .unwrap_or_default();
    }

    let path_to_file_to_unzip = path_to_save_uploaded_file_to.clone();
    task::spawn_blocking(move || {
        // * unzip the file
        let mut archive =
            zip::ZipArchive::new(std::fs::File::open(&path_to_file_to_unzip).unwrap()).unwrap();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = assets_folder.join(file.name());

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File {} comment: {}", i, comment);
                }
            }

            if (&*file.name()).ends_with('/') {
                println!(
                    "File {} extracted to \"{}\"",
                    i,
                    outpath.as_path().display()
                );
                std::fs::create_dir_all(&outpath).unwrap();
            } else {
                println!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.as_path().display(),
                    file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p).unwrap();
                    }
                }
                let mut outfile = std::fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    })
    .await
    .ok();

    // * delete the zip file
    tokio::fs::remove_file(&path_to_save_uploaded_file_to)
        .await
        .unwrap();

    // * add dfx.json file to the unzip folder
    let mut dfx_json = unzip_folder.clone();
    dfx_json.push("dfx.json");
    tokio::fs::write(
        dfx_json,
        r#"{
  "canisters": {
    "www": {
      "frontend": {
        "entrypoint": "www/index.html"
      },
      "source": ["www"],
      "type": "assets"
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "version": 1
}"#,
    )
    .await
    .unwrap_or_default();

    let folder_to_run_dfx_in = unzip_folder.clone();
    task::spawn_blocking(move || {
        // * run dfx deploy
        match starting_cycles {
            Some(starting_cycles) => {
                std::process::Command::new("dfx")
                    .current_dir(folder_to_run_dfx_in)
                    .arg("deploy")
                    .arg("--with-cycles")
                    .arg(starting_cycles.to_string())
                    // ! comment on local
                    .arg("--network")
                    .arg("ic")
                    .output()
                    .expect("failed to deploy websites");
            }
            None => {
                std::process::Command::new("dfx")
                    .current_dir(folder_to_run_dfx_in)
                    .arg("deploy")
                    // ! comment on local
                    .arg("--network")
                    .arg("ic")
                    .output()
                    .expect("failed to deploy websites");
            }
        };
    })
    .await
    .unwrap_or_default();

    // copy the canister_ids.json file in the .dfx/local folder to the current temp folder
    let mut path_to_canister_ids_json = unzip_folder.clone();

    // ! comment this out for mainnet
    // path_to_canister_ids_json.push(".dfx");
    // path_to_canister_ids_json.push("local");

    path_to_canister_ids_json.push("canister_ids.json");

    // extract the www > local value from the canister_ids.json file
    let canister_ids_json_contents = tokio::fs::read_to_string(path_to_canister_ids_json)
        .await
        .unwrap();
    let canister_ids_json_contents: serde_json::Value =
        serde_json::from_str(&canister_ids_json_contents).unwrap();
    // ! for local only. Use the other one for mainnet
    // let canister_id = canister_ids_json_contents["www"]["local"]
    //     .as_str()
    //     .unwrap()
    //     .to_string();
    let canister_id = canister_ids_json_contents["www"]["ic"]
        .as_str()
        .unwrap()
        .to_string();

    // delete the unzip folder
    tokio::fs::remove_dir_all(&unzip_folder).await.unwrap();

    // return the canister id
    Ok(canister_id)
}
