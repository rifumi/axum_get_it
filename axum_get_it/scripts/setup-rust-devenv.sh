#!/bin/bash
#set -eux
#if [ $# -lt 2 ];then
#  echo "error. \$#=$# usage: $0 <user name> <app name>" >> work.txt
#  echo "error command:$0 $@" >> work.txt
#  exit 1
#fi

#USER_NAME=$1
#APP_NAME=$2

## cargo new $APP_NAMEを実行するディレクトリパス
EARTH_PATH=$APP_EARTH_PATH

if [ -d "$EARTH_PATH/$APP_NAME" ];then
  echo 'already setup finished.'
  exit 0
fi

if [ "$(whoami)" != "$USER_NAME" ];then
  echo "error. $(whoami) != $USER_NAME." >> error.txt
  exit 2
fi
if [ ! -d "$EARTH_PATH" ];then
  echo "failure, not found $EARTH_PATH" >> error.txt
  exit 3
fi
if [ -z "$(which cargo)" ];then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=stable -y
fi
source /usr/local/cargo/env
rustup update stable
rustup -V > ${EARTH_PATH}/verinfo.txt
cargo -V >> ${EARTH_PATH}/verinfo.txt
APP_ROOT_PATH=$EARTH_PATH/$APP_NAME
cd $EARTH_PATH
if [ ! -d $APP_ROOT_PATH ];then
  cargo new $APP_NAME
#else
#  #rm $APP_ROOT_PATH/Cargo.toml || :
#  cargo init $APP_NAME
fi
