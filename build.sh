#!/bin/bash
#
shopt -s expand_aliases

alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release
