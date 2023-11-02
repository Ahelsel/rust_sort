#!/bin/bash
# generates a given number of random numbers within a specified range
# Usage: ./generate_numbers.sh <number_of_numbers> <range_start> <range_end>

if [ "$#" -ne 3 ]; then
  echo "Usage: $0 <number_of_numbers> <range_start> <range_end>"
  exit 1
fi

num=$1
start=$2
end=$3

for ((i=0; i<num; i++)); do
  echo $((RANDOM % (end - start + 1) + start))
done > inp.txt

echo "Random numbers generated and written to inp.txt"
