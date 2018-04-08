set -euxo pipefail

main() {
    # test that building the book works
    mdbook build

    linkchecker book

    # first (fast) pass: check that examples compile
    for chapter in $(echo src/*); do
        if [ ! -f $chapter/Cargo.toml ]; then
            continue
        fi

        pushd $chapter
        if [ $(basename $chapter) = 05-led-roulette ]; then
            cargo check --target thumbv7em-none-eabihf
        elif [ $(basename $chapter) = WIP-async-io-the-future ]; then
            popd
            continue
        else
            cargo check
        fi
        popd
    done

    # second (slow) pass: check that examples link
    for chapter in $(echo src/*); do
        if [ ! -f $chapter/Cargo.toml ]; then
            continue
        fi

        pushd $chapter
        if [ $(basename $chapter) = 05-led-roulette ]; then
            cargo build --target thumbv7em-none-eabihf
            cargo build --target thumbv7em-none-eabihf --release
        elif [ $(basename $chapter) = WIP-async-io-the-future ]; then
            popd
            continue
        else
            cargo build
            cargo build --release
        fi
        popd
    done
}

if [ $TRAVIS_BRANCH != master ]; then
    main
fi
