#!/usr/bin/env bash

# Small script to automate typing longer commands

case $1 in
    fg) # Flamegraph
        RUST_LOG=none cargo bench --bench get_desktop_apps -- --profile-time=5
        xdg-open target/criterion/get\ apps/profile/flamegraph.svg
        ;;
    tt) # Time travel
        RUST_LOG=none cargo r --release --features time-travel
        ;;
    ba) # Benchmark Apps
        RUST_LOG=none cargo bench --bench get_desktop_apps
        ;;
esac
