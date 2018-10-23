set -euxo pipefail

main() {
    # install latest mdbook v0.2.x release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/rust-lang-nursery/mdbook \
                    | cut -d/ -f3 \
                    | grep -E '^v0.2.[0-9]+$' \
                    | sort --version-sort \
                    | tail -n1)
    local tag="v0.2.1"
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git rust-lang-nursery/mdBook \
           --tag $tag \
           --target x86_64-unknown-linux-musl

    rustup target add thumbv7em-none-eabihf

    pip install linkchecker --user
}

main
