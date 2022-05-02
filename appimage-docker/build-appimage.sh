#!/bin/bash
run_container() {
    sudo docker run \
    -v $PWD/../:/root/microlaunch/ \
    -v $PWD/cargo_cache/target/:/root/microlaunch/target \
    -v $PWD/cargo_cache/registry/:/usr/local/cargo/registry \
    microlaunch-docker
}

if sudo docker image inspect microlaunch-docker:latest ; [ "$?" -eq 0 ];
then
    run_container
else
    if cd docker && sudo docker build -t microlaunch-docker:latest . ; [ "$?" -eq 0 ];   
    then
        cd ../ && run_container
    else
        echo "Something went wrong building the cotainer, please inspect the output."
        exit 1
    fi
fi
    


