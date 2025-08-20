#!/bin/bash

set -e

rm -rf ./output \
&& mkdir ./output \
&& docker build -t taskrunner-builder . \
&& docker create --name taskrunner-build-container taskrunner-builder \
&& docker cp taskrunner-build-container:/taskrunner ./output/taskrunner \
&& docker rm taskrunner-build-container

ret=$?
if [ $ret -eq 0 ]; then
  echo "✅ Build done"
  exit 0
fi

echo "❌ Build done"
exit 1