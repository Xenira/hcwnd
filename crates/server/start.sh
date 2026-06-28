#!/bin/bash

export SERVER_PORT=8090
export DATABASE_URL="postgresql://postgres:example@localhost:5432/timetable"
export VALKEY_URL=redis://localhost:6379
export RUST_LOG=debug
export URL_SIGNING_SALT=hello
export URL_SIGNING_KEY=secret
cargo run --release
