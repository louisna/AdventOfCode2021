#!/bin/bash

if [ $# -ne 1 ]; then
	>&2 echo "Day number"
	exit 1
fi

mkdir cmd/day-$1
touch cmd/day-$1/example.txt
touch cmd/day-$1/input.txt
cp template.txt cmd/day-$1/main.go