#!/bin/bash
sudo apt install libmysqlclient-dev libpq-dev libsqlite3-0 libsqlite3-dev postgresql postgresql-contrib
rustup default nightly
rustup update && cargo update
cargo install diesel_cli
read -p "DB Username: " user
read -p "DB Password: " pass
pname=${PWD##*/}
sudo service postgresql start
sudo su -c "createuser $user --pwprompt" postgres
sudo su -c "createdb $pname" postgres
echo DATABASE_URL="postgres://$user:$pass@localhost/$pname" > .env
diesel migration run
# Uncomment below if you are starting a new project
#echo 'rocket = "0.3.6"
#rocket_codegen = "0.3.6"
#diesel = { version = "1.0.0", features = ["postgres" "r2d2"] }
#dotenv = "0.9.0"
#tera = "0.11.6"
#frank_jwt = "3.0.0"
#serde = "1.0"
#serde_derive = "1.0"
#pwhash = "0.1"
#rand = "0.4"
#
#
#[dependencies.rocket_contrib]
#version = "*"
#default-features = false
#features = ["tera_templates"]' >> Cargo.toml
cargo build
