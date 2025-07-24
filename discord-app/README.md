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

To add a profile, start the app with `--params-diff-profile "Name of the profile" path/to/params-diff.json`. You can specify as many `--params-diff-profile`s as you want.

## Default cleaner

See [`../default_cleaner.md`](../default_cleaner.md) for details about the included default cleaner.
