set -euxo pipefail

main() {
    # This fetches latest stable release of mdbook
    tag=$(git ls-remote --tags --refs --exit-code https://github.com/rust-lang-nursery/mdBook \
              | cut -d/ -f3 \
              | grep -E '^v[0.1.0-9.]+$' \
              | sort --version-sort \
              | tail -n1)

    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git rust-lang-nursery/mdBook \
           --tag $tag \
           --target x86_64-unknown-linux-musl

    rustup component add thumbv7em-none-eabihf

    pip install linkchecker --user

    # sanity check that a linker is present
    which arm-none-eabi-ld
}

main
