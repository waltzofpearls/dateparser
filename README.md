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
# parse a unix epoch timestamp
$ belt 1511648546 --short
2017-11-25 14:22:26 -0800

# or show the current local datetime
$ belt --short
2021-05-15 22:54:34 -0700
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

[![Crates.io][cratesio-badge]][cratesio-url]
[![Doc.rs][docrs-badge]][docrs-url]

[cratesio-badge]: https://img.shields.io/crates/v/dateparser.svg
[cratesio-url]: https://crates.io/crates/dateparser
[docrs-badge]: https://docs.rs/dateparser/badge.svg
[docrs-url]: https://docs.rs/crate/dateparser/

Date parsing in belt is powered by `dateparser` crate, which is [a part of this repo](./dateparser/).

## Accepted date formats

Date string in the following formats can be parsed by `belt`:

```
1511648546
1620021848429
1620024872717915000
2021-05-01T01:17:02.604456Z
2017-11-25T22:34:50Z
Wed, 02 Jun 2021 06:31:39 GMT
2019-11-29 08:08:05-08
2021-05-02 23:31:36.0741-07
2021-05-02 23:31:39.12689-07
2019-11-29 08:15:47.624504-08
2021-04-30 21:14:10
2021-04-30 21:14:10.052282
2017-11-25 13:31:15 PST
2017-11-25 13:31 PST
2021-02-21
2021-02-21 PST
01:06:06
4:00pm
6:00 AM
01:06:06 PST
4:00pm PST
6:00 AM PST
May 02, 2021 15:51:31 UTC
May 02, 2021 15:51 UTC
```
