#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if ! command -v pbc &>/dev/null; then
  cargo install pbc
fi
pbc
mv rust/proto_tran/src/proto_tran.rs proto_tran/src/
rm -rf pb proto__ rust

echo -e '\npub use proto_tran::*;' >>proto_tran/src/lib.rs

cd ./proto_tran
touch Cargo.lock
rm Cargo.toml
ln -s ../proto_tran.toml Cargo.toml
awk '!seen[$0]++' src/lib.rs | sponge src/lib.rs
