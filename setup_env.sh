#!/bin/bash
sudo apt install libmysqlclient-dev
sudo apt install libpq-dev
rustup default nightly
rustup update && cargo update
cargo install diesel_cli
read -p "DB Username: " user
read -p "DB Password: " pass
pname=${PWD##*/}
sudo service postgresql start
sudo su - postgres
createuser $user
createdb $pname
logout
diesel setup
echo DATABASE_URL="postgres://$user:$pass@localhost/$pname" > .env
echo 'rocket = "0.3.6"
rocket_codegen = "0.3.6"
diesel = { version = "1.0.0", features = ["postgres"] }
r2d2-diesel = "1.0"
r2d2 = "0.8"
dotenv = "0.9.0"
tera = "0.11.6"
frank_jwt = "3.0.0"
serde = "1.0"
serde_derive = "1.0"


[dependencies.rocket_contrib]
version = "*"
default-features = false
features = ["tera_templates"]' >> Cargo.toml
cargo build
