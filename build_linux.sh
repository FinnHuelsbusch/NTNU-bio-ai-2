#!/bin/bash
export MALLOC_CONF="thp:always,metadata_thp:always"
cargo build --release
