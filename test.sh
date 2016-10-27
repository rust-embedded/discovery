#!/bin/sh

set -ex

for d in src/*; do
    if [ -f $d/Cargo.toml ]; then
        cd $d
        xargo build --target thumbv7em-none-eabihf
        rm -rf target
        cd ../..
    fi
done
