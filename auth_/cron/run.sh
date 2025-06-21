#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

bun x envexpand \
  ../../../conf/srv/r.env \
  ../../../conf/srv/cron/_apiToken.env \
  ../../../conf/srv/cron/denoNotifyApi.env \
  ../../../conf/srv/cron/_pg.env >/tmp/authCron.env
deno=$(which deno)
set -ex

env - $deno eval --env-file=/tmp/authCron.env --unstable-cron \
  "import cron_once from 'jsr:@8v/cron-once';await cron_once(import('./main.js'));"
