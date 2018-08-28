set -euxo pipefail

main() {
    # install latest mdbook v0.2.x release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/rust-lang-nursery/mdbook \
                    | cut -d/ -f3 \
                    | grep -E '^v0.2.[0-9]+$' \
                    | sort --version-sort \
                    | tail -n1)
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git rust-lang-nursery/mdBook \
           --tag $tag \
           --target x86_64-unknown-linux-musl

    rustup target add thumbv7em-none-eabihf

    pip install linkchecker --user

    # install gcc
    mkdir gcc

    curl -L https://developer.arm.com/-/media/Files/downloads/gnu-rm/7-2018q2/gcc-arm-none-eabi-7-2018-q2-update-linux.tar.bz2?revision=bc2c96c0-14b5-4bb4-9f18-bceb4050fee7?product=GNU%20Arm%20Embedded%20Toolchain,64-bit,,Linux,7-2018-q2-update | tar --strip-components=1 -C gcc -xj
}

main
