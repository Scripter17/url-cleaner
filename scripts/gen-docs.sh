#!/usr/bin/bash

echo "#### Flags"
echo ""
echo "Flags let you specify behaviour with the \`--flag name --flag name2\` command line syntax."
echo ""
echo "Various flags are included in the default config for things I want to do frequently."
echo ""
cat default-config.json | jq '.docs.flags | to_entries | sort_by(.key) | map(("- `" + .key + "`: " + .value)) | join("\n")' -r
echo ""
echo "If a flag is enabled in a config's \`params\` field, it can be disabled using \`--unflag flag1 --unflag flag1\`."
echo ""

echo "#### Variables"
echo ""
echo "Variables let you specify behaviour with the \`--var name value --var name2 value2\` command line syntax."
echo ""
echo "Various variables are included in the default config for things that have multiple useful values."
echo ""
cat default-config.json | jq '.docs.vars | to_entries | sort_by(.key) | map(("- `" + .key + "`: " + .value)) | join("\n")' -r
echo ""
echo "If a variable is specified in a config's \`params\` field, it can be unspecified using \`--unvar var1 --unvar var2\`."
echo ""

echo "#### Environment variables"
echo ""
echo "There are some things you don't want in the config, like API keys, that are also a pain to repeatedly insert via \`--var abc xyz\`. For this, URL Cleaner uses environment variables."
echo ""
cat default-config.json | jq '.docs.environment_vars | to_entries | sort_by(.key) | map(("- `" + .key + "`: " + .value)) | join("\n")' -r
echo ""

echo "#### Sets"
echo ""
echo "Sets let you check if a string is one of many specific strings very quickly."
echo ""
echo "Various sets are included in the default config."
echo ""
cat default-config.json | jq '.docs.sets | to_entries | sort_by(.key) | map(("- `" + .key + "`: " + .value)) | join("\n")' -r
echo ""
echo "Sets can have elements inserted into them using \`--insert-into-set name1 value1 value2 --insert-into-set name2 value3 value4\`."
echo ""
echo "Sets can have elements removed from them using \`--remove-from-set name1 value1 value2 --remove-from-set name2 value3 value4\`."
echo ""

echo "#### Lists"
echo ""
echo "Lists allow you to iterate over strings for things like checking if another string contains any of them."
echo ""
echo "Currently only one list is included in the default config:"
echo ""
cat default-config.json | jq '.docs.lists | to_entries | sort_by(.key) | map(("- `" + .key + "`: " + .value)) | join("\n")' -r
echo ""
echo "Currently there is no command line syntax for them. There really should be."
