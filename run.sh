#!/usr/bin/env bash

case $1 in
    fg)
        cargo bench --bench get_desktop_apps -- --profile-time=5
        xdg-open target/criterion/get\ apps/profile/flamegraph.svg
        ;;
    tt)
        cargo r --release --features time-travel
esac
