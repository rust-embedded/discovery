set -euxo pipefail

main() {
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git rust-lang-nursery/mdBook \
           --tag v0.1.5 \
           --target x86_64-unknown-linux-musl

    rustup target add thumbv7em-none-eabihf

    pip install linkchecker --user

    # sanity check that a linker is present
    which arm-none-eabi-ld
}

main
