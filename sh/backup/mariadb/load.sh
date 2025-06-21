#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -a
. ../../../../srv/conf/mariadb.env
set +a
set -xe
bun i
exec mise exec -- ./load.coffee
