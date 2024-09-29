#!/usr/bin/env sh

set -e

dagger call \
    simple-example-service --source=. --local-dev-service=tcp://localhost:8080 \
    up --ports 8090:8090 \
    &> /tmp/example_service_log &

example_service_pid="$!"

dagger call \
    local-dev-service --source=. --example-service=tcp://localhost:8090 \
    up --ports 8080:8080 \
    &> /tmp/local_dev_service_log &

local_dev_service_pid="$!"

dagger call drive-integration-tests --source=integration --local-dev-service=tcp://localhost:8080

kill $example_service_pid
kill $local_dev_service_pid
