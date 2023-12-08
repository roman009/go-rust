#!/usr/bin/env bash

./build.sh || exit 1

./deploy.sh || exit 1

echo "Done."
