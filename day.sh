#!/bin/sh
[ $# -lt 1 ] && echo "missing day number" && exit 1
ROOT="$(pwd)"

mkdir -p $ROOT/assets/day$1
cd $ROOT/assets/day$1
touch input.txt
cd $ROOT/src/bin
touch day$1.rs
