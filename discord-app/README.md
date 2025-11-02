# URL Cleaner Discord App

A basic discord app to use URL Cleaner.

Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)

https://www.gnu.org/licenses/agpl-3.0.html

## Usage

The app looks for a discord app token in the `URLCDA_KEY` environment variable and prints a URL to install the app to your account.

Once installed, you can right click any message (even in DMs), click "Apps", then click "Clean URLs". The app will then display a message only you can see with each detected URL cleaned.

Please note that currently spoiler tags are not preserved and code blocks are not ignored.

### Profiles

Because context menu actions can't take arguments, URL Cleaner Discord App has "profiles" that let you add additional "Clean URLs" buttons with different ParamsDiffs.

The ProfileConfig is a JSON file like this:

```Json
{
  "base": {
    "params_diff": {
      "flags": ["flag you always want"]
    }
  },
  "profiles": {
    "profile name 1": {
      "params_diff": {
        "flags": ["flag you only sometimes want"]
      }
    }
  }
}
```

Using the base profile will enable the `flag you always want` flag, while using `profile name 1` will also enable the `flag you only sometimes want` flag.

## Bundled cleaner

See [`../bundled_cleaner.md`](../bundled_cleaner.md) for details about the included bundled cleaner.
