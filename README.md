# belt + dateparser

A CLI app to show your time from a list of selected time zones, and a rust lib to parse dates in string
formats that are commonly used.

Run `belt` command to parse an input date and display the parsed date in these time zones by default:

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

Or display in the short form

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
