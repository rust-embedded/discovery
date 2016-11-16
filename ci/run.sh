set -ex

test_mode() {
    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        mdbook build
        linkchecker book
    fi

    for chapter in $(echo src/*); do
        if [ ! -f $chapter/Cargo.toml ]; then
            continue
        fi

        cd $chapter
        xargo build --target thumbv7em-none-eabihf
        xargo build --target thumbv7em-none-eabihf --release
        cd ../..
    done
}

run() {
    test_mode
}

run
