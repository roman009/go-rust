#!/usr/bin/env bash

./build.sh || exit 1

./install.sh || exit 1

echo "Done."
