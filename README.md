# Serial Unit Testing

[![Build Status](https://travis-ci.org/markatk/serial-unit-testing.svg?branch=master)](https://travis-ci.org/markatk/serial-unit-testing)
[![Build status](https://ci.appveyor.com/api/projects/status/7b9osucvqpqp9ebd/branch/master?svg=true)](https://ci.appveyor.com/project/markatk/serial-unit-testing/branch/master)

# Description

Serial unit testing program and library written in rust

## Script syntax

Scripts can be executed with the `run` command or parsed with `parser::parse_file`. They must follow the following syntax:

Each line represents a test, test group or comment:

## Comment

Each line starting with the hash sign `#` will be treated as a comment and ignored by the parser.

Example: `# This is a comment`

## Group

Test can be grouped to separate them visually and optionally add group attributes to them. A group line starts with `[` followed by the group name and ends with `]`. All tests following the group line will belong to this group as long as no other group is introduced.
If no group is present the tests will be independent.

Example: `[Group One]`

## Test

Each line not being a comment or group (and not being empty) is interpreted as a test. Test have an input and an output part which must both be present and are separated with a colon `:`. Tests may start with a name surrounded with brackets `()`, otherwise the input content is used as the test display name. The input/output content must be surrounded with quotation marks `""`. Optionally the content can be prefixed with one of the following format specifier, otherwise text mode is used. Input and output format specifiers can be different.

 - `b`: Binary mode
 - `o`: Octal mode
 - `d`: Decimal mode
 - `h`: Hexadecimal mode

Content in text mode can be escaped with a backslash `\` thus characters like `"` can be included.

Example: `(Test One)h"58990d" : "OK\r"`

## Example Script

```
# Example script

(Independent Test One)"h\n" : "Help"
(Independent Test Two)h"00FF" : h"00"
h"00af" : h"03"

[Group One]
(Group Test One)"gp\n" : "yes"
(Group Test Two)"gq\n" : "no"
```
