#!/bin/bash

# export IMGPROXY_KEY=$(xxd -g 2 -l 64 -p /dev/random | tr -d '\n')
# export IMGPROXY_SALT=$(xxd -g 2 -l 64 -p /dev/random | tr -d '\n')
export IMGPROXY_KEY="736563726574"
export IMGPROXY_SALT="68656c6c6f"
export HTTP_PORT=8080
export HTTPS_PORT=8443
export IP_ADDRESS=$(ip addr show scope global | awk '$1 ~ /^inet/ {print $2}' | cut -d/ -f1 | head -n 1)
docker compose up $@
