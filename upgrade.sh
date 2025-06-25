#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

ncu -u
cd proto_tran

cargo upgrade -i --recursive --verbose
