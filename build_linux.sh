#!/bin/bash
export MALLOC_CONF="thp:always,metadata_thp:always"
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release
./target/release/bio-ai-2 ./configs/pgo_config.json     
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release