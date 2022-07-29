#!/bin/bash

if [ $# != 1 ]; then
  echo "USAGE: ./udp.sh port [default: 16523]"
  exit 1
fi

echo "bash run"

while true; do 
  nc -lu $1
done