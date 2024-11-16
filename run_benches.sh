#!/bin/bash

function run_bench {
    # reset baseline. Importantly, this resets some calibration data
    # which iai seems to share between benches
    cargo $1 clean
    cargo $1 bench --bench iai_callgrind > /dev/null
    cargo $1 bench --bench iai_callgrind
}

run_bench +1.78
run_bench +1.79