@echo off

ren Cargo.toml Cargo-bak.toml
ren c_api.toml Cargo.toml

cargo +stable-i686-pc-windows-msvc build --release

ren Cargo.toml c_api.toml
ren Cargo-bak.toml Cargo.toml

cbindgen --config cbindgen.toml --crate replay --output replay.h

mkdir ..\..\OmegaBotUI\replay
copy replay.h ..\..\OmegaBotUI\replay\replay.h
copy target\release\replay.lib ..\..\OmegaBotUI\replay\replay.lib
