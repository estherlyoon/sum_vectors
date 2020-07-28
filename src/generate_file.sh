#!/bin.bash

while true; do
	awk -v n=8 -v seed="$RANDOM" 'BEGIN { srand(seed); for (i=0; i<n; i++) printf("%d%.4f,", rand()*100, rand()); printf("%d%.4f\n", rand()*100, rand()); }' >> input.txt
done
