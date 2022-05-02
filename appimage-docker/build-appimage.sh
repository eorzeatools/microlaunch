#!/bin/bash

# As for the three volumes defined below:
# 1. Mount the source tree into the container for direct usage.
# 2. Mount a local directoy for cargo build cache, so we can reuse it.
# 3. Mount a local directory for cargo registry cache, this will probably break at some point?
run_container() {
    sudo docker run \
    -v $PWD/../:/root/microlaunch/ \
    -v $PWD/cargo_cache/target/:/root/microlaunch/target \ 
    -v $PWD/cargo_cache/registry/:/usr/local/cargo/registry \ 
    microlaunch-docker
}

# Check if microlaunch-dcker container already exists. (exit code 0)
if sudo docker image inspect microlaunch-docker:latest ; [ "$?" -eq 0 ];
then
    run_container

# If not, we build it, and check if build succeeds. (exit code 0)
else
    if cd docker && sudo docker build -t microlaunch-docker:latest . ; [ "$?" -eq 0 ];
    then
        cd ../ && run_container
    else
        echo "Something went wrong building the cotainer, please inspect the output."
        exit 1
    fi
fi
    


