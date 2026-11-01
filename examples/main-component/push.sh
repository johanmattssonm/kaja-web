#!/bin/sh
set -eu

./build.sh

: "${DEPLOY_USER:?Set DEPLOY_USER to the SSH user on git.kajacode.com}"
: "${DEPLOY_PATH:?Set DEPLOY_PATH to the Caddy-served folder}"

rsync -av --delete \
    pkg/ \
    "${DEPLOY_USER}@git.kajacode.com:${DEPLOY_PATH}/"

if [ -n "${GIT_REMOTE:-}" ]; then
    git push "${GIT_REMOTE}" "${GIT_BRANCH:-main}"
fi
