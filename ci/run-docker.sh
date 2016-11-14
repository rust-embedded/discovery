set -ex

run() {
    # This directory needs to exist before calling docker, otherwise docker will create it but it
    # will be owned by root
    mkdir -p target

    docker build -t $1 ci/docker/$1
    docker run \
           --rm \
           --user $(id -u):$(id -g) \
           -e CARGO_HOME=/cargo \
           -e DEPLOY_VERSION=$DEPLOY_VERSION \
           -e TARGET=$1 \
           -e TRAVIS_OS_NAME=linux \
           -e TRAVIS_RUST_VERSION=$TRAVIS_RUST_VERSION \
           -e TRAVIS_TAG=$TRAVIS_TAG \
           -e USER=$USER \
           -v $HOME/.cargo:/cargo \
           -v `pwd`/target:/target \
           -v `pwd`:/checkout \
           -v `rustc --print sysroot`:/rust:ro \
           -w /checkout \
           -it $1 \
           sh -c "HOME=/tmp PATH=\$PATH:/rust/bin ci/run.sh"
}

if [ -z $1 ]; then
    for d in `ls ci/docker/`; do
        run $d
    done
else
    run $1
fi
