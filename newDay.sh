#!/usr/bin/sh

day=$1

mkdir $day
cp -r ./.template/* $day

set -a; 
. ./.env; 
set +a;

curl -s --cookie "session=$AOC_SESSION" https://adventofcode.com/2022/day/$day/input >> $day/input.TXT
