#!/usr/bin/bash

for file in *.md */*.md; do

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
  done < $file > $file.temp
  rm $file
  mv $file.temp $file

done
