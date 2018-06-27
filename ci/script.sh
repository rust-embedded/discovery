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
        case $(basename $chapter) in
            05-led-roulette | 06-hello-world)
                cargo check --target thumbv7em-none-eabihf
                ;;
            WIP-async-io-the-future)
                popd
                continue
                ;;
            *)
                cargo check
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
