#!/usr/bin/bash

printlines=1

while IFS= read -r line; do
  if cmd=$(echo $line | grep -oP "(?<=<!--cmd ).+(?=-->)"); then
    echo "$line"
    bash -c "$cmd"
    printlines=0
  elif echo $line | grep "<!--/cmd-->"; then
    printlines=1
  elif [ $printlines -eq 1 ]; then
    echo "$line"
  fi
done < README.md > README.md.temp
rm README.md
mv README.md.temp README.md
