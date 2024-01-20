#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <command> <times>"
    exit 1
fi

command=$1
times=$2
total=0

for ((i=1; i<=times; i++))
do
   start=$(date +%s%N)
   $command > /dev/null
   end=$(date +%s%N)
   total=$((total+end-start))
done

average=$(echo "scale=2; $total/$times/1000000" | bc)
echo "Average execution time: $average ms"