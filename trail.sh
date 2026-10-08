#!/bin/bash
# 
while IFS= read -r -n 1 byte_char; do
    if [ -z "$byte_char" ]; then
        echo "Found a NULL byte (0x00)"
    else
        echo "$byte_char"
    fi
done < "/proc/softirqs" >> result.txt