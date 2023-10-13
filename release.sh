#!/bin/sh
set -e
if [ "$(which py > /dev/null 2>&1)" = 0 ]; then
    echo "Running in Windows"
    PYTHON='python'
    py -3 -m venv venv
    . venv/Scripts/activate
else
    echo "Running in Linux"
    PYTHON=python3
    python3 -m venv venv
    . venv/bin/activate
fi

git stash

${PYTHON} -m pip install bump2version
git fetch
git checkout dev
git pull
git checkout main
git pull
git merge dev
bump2version minor
if [ -n "${SKIP_PUSH}" ]; then
  git push origin main --tags
fi
