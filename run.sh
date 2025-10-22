#!/usr/bin/env bash

case $1 in
    fg)
        cargo bench --bench get_desktop_app -- --profile-time=5
        xdg-open target/criterion/get\ apps/profile/flamegraph.svg
        ;;
    tt)
        cargo r --features time-travel
esac
