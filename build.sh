#!/usr/bin/env bash

# Build the projects and docker images

COMMIT_HASH=$(git rev-parse --short HEAD)

echo "Building rust-main with commit hash $COMMIT_HASH"

docker build -t rust-main:latest -t rust-main:$COMMIT_HASH -t localhost:32000/rust-main:$COMMIT_HASH -t localhost:32000/rust-main:latest -f apps/rust-main/rust-main.Dockerfile apps/rust-main/ || exit 1

docker push localhost:32000/rust-main:$COMMIT_HASH || exit 1

docker push localhost:32000/rust-main:latest || exit 1


echo "Building go-main with commit hash $COMMIT_HASH"

docker build -t go-main:latest -t go-main:$COMMIT_HASH -t localhost:32000/go-main:$COMMIT_HASH -t localhost:32000/go-main:latest -f apps/go-main/go-main.Dockerfile apps/go-main/ || exit 1

docker push localhost:32000/go-main:$COMMIT_HASH || exit 1

docker push localhost:32000/go-main:latest || exit 1


echo "Building kill-job with commit hash $COMMIT_HASH"

docker build -t kill-job:latest -t kill-job:$COMMIT_HASH -t localhost:32000/kill-job:$COMMIT_HASH -t localhost:32000/kill-job:latest -f apps/kill-job/kill-job.Dockerfile apps/kill-job/ || exit 1

docker push localhost:32000/kill-job:$COMMIT_HASH || exit 1

docker push localhost:32000/kill-job:latest || exit 1


echo "Building service-discoverer with commit hash $COMMIT_HASH"

docker build -t service-discoverer:latest -t kill-job:$COMMIT_HASH -t localhost:32000/service-discoverer:$COMMIT_HASH -t localhost:32000/service-discoverer:latest -f apps/service-discoverer/service-discoverer.Dockerfile apps/service-discoverer/ || exit 1

docker push localhost:32000/service-discoverer:$COMMIT_HASH || exit 1

docker push localhost:32000/service-discoverer:latest || exit 1


echo "Done building and pushing."
