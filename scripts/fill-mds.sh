#!/usr/bin/bash

# Fill out all the markdown files using the `<!--cmd (command goes here)-->`/`<!--/cmd-->` syntaax.

cd $(dirname "$0")/..

for file in *.md */*.md; do
  printlines=1

  while IFS= read -r line; do
    if [ $printlines -eq 1 ]; then
      echo "$line"
    fi

    if cmd=$(echo "$line" | grep -oP '(?<=<!--cmd ).+(?=-->)'); then
      bash -c "$cmd"
      printlines=0
    elif [ "$line" == '<!--/cmd-->' ]; then
      echo "$line"
      printlines=1
    fi
  done < "$file" > "$file.temp"
  rm "$file"
  mv "$file.temp" "$file"
done

cd - &> /dev/null
