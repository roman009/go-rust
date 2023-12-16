#!/usr/bin/env bash

microk8s helm3 delete rust-main
microk8s helm3 delete go-main
microk8s helm3 delete kill-job
microk8s helm3 delete service-discoverer
microk8s helm3 delete kafka

echo "Done uninstalling."