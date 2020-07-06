set -euxo pipefail

main() {
    # test that building the book works
    mdbook build

    linkchecker book

    # now check this as a directory of the bookshelf
    rm -rf shelf
    mkdir shelf
    mv book shelf
    linkchecker shelf

    mv shelf/book .
    rmdir shelf

    # first (fast) pass: check that examples compile
    for chapter in $(echo src/*); do
        if [ ! -f $chapter/Cargo.toml ]; then
            continue
        fi

        pushd $chapter
        case $(basename $chapter) in
            05-led-roulette | 06-hello-world)
                RUSTFLAGS="-D rust_2018_compatibility -D rust_2018_idioms" cargo check --target thumbv7em-none-eabihf
                ;;
            WIP-async-io-the-future)
                popd
                continue
                ;;
            *)
                RUSTFLAGS="-D rust_2018_compatibility -D rust_2018_idioms" cargo check
                ;;
        esac
        popd
    done

    # second (slow) pass: check that examples link
    for chapter in $(echo src/*); do
        if [ ! -f $chapter/Cargo.toml ]; then
            continue
        fi

        pushd $chapter
        case $(basename $chapter) in
            05-led-roulette | 06-hello-world)
                cargo build --target thumbv7em-none-eabihf
                cargo build --target thumbv7em-none-eabihf --release
                ;;
            WIP-async-io-the-future)
                popd
                continue
                ;;
            *)
                cargo build
                cargo build --release
                ;;
        esac
        popd
    done
}

if [ $TRAVIS_BRANCH != master ]; then
    main
fi
