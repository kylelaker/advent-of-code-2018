#!/usr/bin/env bash

if [ -z "$1" ]; then
    echo "!!! No day specified"
fi

day="$1"

cargo init --bin "$day"
cp .gitignore "$day/.gitignore"
mkdir "$day/input"
