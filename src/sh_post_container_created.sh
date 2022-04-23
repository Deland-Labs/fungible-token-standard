#!/bin/bash
echo "Script executed from: ${PWD}"

BASEDIR=$(dirname $0)
echo "Script location: ${BASEDIR}"
cd $(dirname $0)
pre-commit install
dfx start --background
./sh_setup_dev.sh
dfx stop
