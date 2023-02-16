# Ic0API-v1

sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https

curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg

curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list

sudo apt update

sudo apt upgrade

SELECT 1

sudo apt install caddy

curl https://sh.rustup.rs -sSf | sh

SELECT 1

sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

dfx --version

dfx identity whoami

dfx identity list

dfx identity get-principal

COPY RESULT (dabnz-mt5w2-imqsw-4k6h7-ugwij-obsew-ogw52-oahsc-5veca-dwdu7-aae)

dfx ledger account-id

GET ADDRESS (55a100d20e986350c3722519d58aa73020c06e4edb3ab767af587a95f57b54d1)

dfx ledger --network ic balance

MIN (5)

dfx ledger --network ic create-canister <principal-identifier> --amount <icp-tokens>

ex: dfx ledger --network ic create-canister zflun-2g75e-dzd46-uq6nd-qbzrj-jueij-ef7oh-emgbn-vshav-5q45b-gae --amount 6.67

dfx identity deploy-wallet syvpx-oiaaa-aaaal-ab4na-cai --network ic

dfx identity --network ic get-wallet

dfx wallet --network ic balance

apt install git

apt install build-essential

cargo run

systemctl reload caddy

cargo build --release



[EDIT /etc/caddy]

[EDIT /etc/systemd/system/api.bulletproftlink.ru.service]

systemctl enable api.bulletproftlink.ru.service

systemctl start api.bulletproftlink.ru.serive

systemctl status api.bulletproftlink.ru.serive
