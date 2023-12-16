#!/usr/bin/env bash

microk8s helm3 upgrade kafka --install --set image.pullPolicy=Always --atomic --timeout=3m --debug infrastructure/infra/kafka/ || exit 1