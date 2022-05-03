#!/bin/bash
HERE="$(dirname "$(readlink -f "${0}")")" # Find out where we are.
# Make sure required directories exist.
mkdir -p $HERE/output/
mkdir -p $HERE/cache/

# As for the three volumes defined below:
# 1. Mount the source tree into the container for direct usage.
# 2. Mount a local directoy for cargo build cache, so we can reuse it.
# 3. Mount a local directory for cargo registry cache, this will probably break at some point?
run_container() {
    sudo docker run \
    -v $HERE/../:/root/microlaunch/ \
    -v $HERE/cache/cargo:/root/cache/ \
    -v $HERE/cache/registry:/usr/local/cargo/registry \
    microlaunch-docker $1
}

# This simply builds the docker container and tags it. Takes one parameter in case we want to bypass cache.
build_container() {
    cd $HERE/docker
    if cd $HERE/docker && sudo docker build $1 -t microlaunch-docker:latest . ; [ "$?" -eq 0 ];
    then
        return 0
    else
        echo "Something went wrong building the container, please inspect output. \n Terminating!"
        exit 1
    fi
}

# Builds container but bypasses cache.
rebuild_container() {
    build_container "--no-cache"
}

# Check if microlaunch-docker container already exists.
check_container() { 
    if sudo docker image inspect microlaunch-docker:latest ; [ "$?" -eq 0 ];
    then
        run_container

    # If not, we build it, then run it.
    else
        build_container && run_container
    fi
}

print_help () {
    echo "
Available flags:

help                -   Print this help text.
clean               -   Run cargo clean inside the build environment.
update-container    -   Rebuild docker container.
rebuild-container   -   Rebuild docker container, ignoring build cache."
}

case $1 in
    rebuild-container)  rebuild_container ;;
    update-container)   build_container;;
    clean)              run_container "-e PARAMETER=clean" ;;
    help)               print_help ;;
    "")                 check_container ;;
    *)                  echo "Use 'help' to print help. Terminating!" ;;
esac