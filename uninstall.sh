#!/usr/bin/env bash

microk8s helm3 delete rust-main go-main kill-job service-discoverer


echo "Done uninstalling."