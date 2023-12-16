#!/usr/bin/env bash

microk8s helm3 upgrade kafla --install --set image.pullPolicy=Always --atomic --timeout=1m infrastructure/infra/kafka/ || exit 1