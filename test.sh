#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

if [ ${#1} -eq 0 ]; then
  if [ -f ".project" ]; then
    arg=$(cat .project)
  else
    echo "❯ $0 项目名"
    exit 1
  fi
else
  echo $@ >.project
  arg=$@
fi

. ../sh/rustflag.sh

set -ex

cd $arg
cargo nextest run --all-features --nocapture
