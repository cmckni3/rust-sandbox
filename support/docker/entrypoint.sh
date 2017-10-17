#!/bin/bash
set -x

APP_ENV=${APP_ENV:-dev}

if [[ ${APP_ENV} == "dev" ]]; then
  cargo build
fi

cargo run
