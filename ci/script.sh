set -euxo pipefail

main() {
    # first (fast) pass: check that examples compile
    for chapter in $(echo src/*); do
        if [ ! -f $chapter/Cargo.toml ]; then
            continue
        fi

        pushd $chapter
        if [ $(basename $chapter) = 05-led-roulette ]; then
            xargo check --target thumbv7em-none-eabihf
        elif [ $(basename $chapter) = WIP-async-io-the-future ]; then
            popd
            continue
        else
            xargo check
        fi
        popd
    done

    # test that building the book works
    mdbook build

    linkchecker book

    # second (slow) pass: check that examples link
    for chapter in $(echo src/*); do
        if [ ! -f $chapter/Cargo.toml ]; then
            continue
        fi

        pushd $chapter
        if [ $(basename $chapter) = 05-led-roulette ]; then
            xargo build --target thumbv7em-none-eabihf
            xargo build --target thumbv7em-none-eabihf --release
        elif [ $(basename $chapter) = WIP-async-io-the-future ]; then
            popd
            continue
        else
            xargo build
            xargo build --release
        fi
        popd
    done
}

if [ $TRAVIS_BRANCH != master ]; then
    main
fi
