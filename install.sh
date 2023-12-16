#!/usr/bin/env bash

COMMIT_HASH=$(git rev-parse --short HEAD)

microk8s helm3 upgrade go-main --install --set image.tag=$COMMIT_HASH --set image.pullPolicy=Always --atomic --timeout=1m infrastructure/apps/go-main/ || exit 1

microk8s helm3 upgrade rust-main --install --set image.tag=$COMMIT_HASH --set image.pullPolicy=Always --atomic --timeout=1m infrastructure/apps/rust-main/ || exit 1

microk8s helm3 upgrade kill-job --install --set image.tag=$COMMIT_HASH --set image.pullPolicy=Always --atomic --timeout=1m infrastructure/apps/kill-job/ || exit 1

microk8s helm3 upgrade service-discoverer --install --set image.tag=$COMMIT_HASH --set image.pullPolicy=Always --atomic --timeout=1m infrastructure/apps/service-discoverer/ || exit 1


echo "Done deploying."