set -euxo pipefail

main() {
    mdbook build

    bash ga.sh

    mkdir ghp-import

    curl -Ls https://github.com/davisp/ghp-import/archive/master.tar.gz |
        tar --strip-components 1 -C ghp-import -xz

    ./ghp-import/ghp_import.py book

    set +x
    git push -fq https://$GH_TOKEN@github.com/$TRAVIS_REPO_SLUG.git gh-pages && echo OK
}

if [ $TRAVIS_BRANCH = master ]; then
    main
fi
