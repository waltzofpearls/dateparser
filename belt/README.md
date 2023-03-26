# `belt` CLI tool

Command-line app that can show your time from a list of selected time zones. It uses `dateparser`
rust crate to parse date strings in commonly used formats.

## Installation

MacOS Homebrew or Linuxbrew:

```shell
brew tap waltzofpearls/belt
brew install belt
```

## Run `belt` to parse a given date

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

## Display parsed date in the short form

```shell
# parse a unix epoch timestamp
$ belt 1511648546 --short
2017-11-25 14:22:26 -0800

# or show the current local datetime
$ belt --short
2021-05-15 22:54:34 -0700
```

## Configure time zone

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
