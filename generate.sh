#!/usr/bin/env bash

# install deps
cargo install protoc-gen-tonic
cargo install protoc-gen-prost-crate
cargo install protoc-gen-prost

# change into current dir
cd "$(dirname "$0")"

# clean everything so we start fresh
rm -rf generated/rust
mkdir -p generated/rust

protoc \
    --prost_out=generated/rust \
    --tonic_out=generated/rust \
    --prost-crate_out=generated/rust \
    --prost-crate_opt=gen_crate=templates/Cargo.toml \
    protos/foo/foo.proto

protoc \
    --prost_out=generated/rust \
    --tonic_out=generated/rust \
    --prost-crate_out=generated/rust \
    --prost-crate_opt=gen_crate=templates/Cargo.toml \
    protos/bar/bar.proto