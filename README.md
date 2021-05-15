# belt + dateparser

[![Build Status][actions-badge]][actions-url]
[![MIT licensed][mit-badge]][mit-url]

[actions-badge]: https://github.com/waltzofpearls/belt/workflows/ci/badge.svg
[actions-url]: https://github.com/waltzofpearls/belt/actions?query=workflow%3Aci+branch%3Amain
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/waltzofpearls/belt/blob/main/LICENSE

`belt` is a command line app that can show your time from a list of selected time zones. `dateparser`
is a rust library for parsing date strings in commonly used formats.

Run `belt` to parse a given date:

```shell
$ belt 'MAY 12, 2021 16:44 UTC'
+-------------------+---------------------------+
| Zone              | Date & Time               |
+===================+===========================+
| Local             | 2021-05-12 09:44:00 -0700 |
|                   | 1620837840                |
+-------------------+---------------------------+
| UTC               | 2021-05-12 16:44:00 +0000 |
|                   | 2021-05-12 16:44 UTC      |
+-------------------+---------------------------+
| America/Vancouver | 2021-05-12 09:44:00 -0700 |
|                   | 2021-05-12 09:44 PDT      |
+-------------------+---------------------------+
| America/New_York  | 2021-05-12 12:44:00 -0400 |
|                   | 2021-05-12 12:44 EDT      |
+-------------------+---------------------------+
| Europe/London     | 2021-05-12 17:44:00 +0100 |
|                   | 2021-05-12 17:44 BST      |
+-------------------+---------------------------+
```

Display parsed date in the short form:

```shell
$ belt 'MAY 12, 2021 16:44 UTC' --short
2021-05-12 09:44:00 -0700
```

Time zones are configurable:

```shell
$ belt config --help
belt-config
Configure time zones list

USAGE:
    belt config [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -l, --list       List existing time zones
    -r, --reset      Reset to default list of time zones
    -V, --version    Prints version information

OPTIONS:
    -a, --add <timezone_to_add>          Add a new time zone to the list
    -d, --delete <timezone_to_delete>    Delete a time zone from the list
```

## Installation

MacOS Homebrew or Linuxbrew:

```shell
brew tap waltzofpearls/belt
brew install belt
```

## `dateparser`

[![Crate][crate-badge]][crate-url]

[crate-badge]: https://img.shields.io/crates/v/dateparser.svg
[crate-url]: https://crates.io/crates/dateparser

Date parsing in belt is powered by `dateparser` crate, which is [a part of this repo](./dateparser/).
