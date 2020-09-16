set -euxo pipefail

main() {
    local tag=$(git ls-remote --tags --refs --exit-code \
                    https://github.com/rust-lang/mdbook \
                    | cut -d/ -f3 \
                    | grep -E '^v[0-9\.]+$' \
                    | sort --version-sort \
                    | tail -n1)

    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git rust-lang/mdbook \
           --tag $tag

    rustup target add thumbv7em-none-eabihf
    rustup target add thumbv6m-none-eabi

    pip install linkchecker --user
}

main
