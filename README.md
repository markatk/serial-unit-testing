# Serial Unit Testing

![Crates.io](https://img.shields.io/crates/v/serial-unit-testing.svg)
[![Build Status](https://travis-ci.org/markatk/serial-unit-testing.svg?branch=master)](https://travis-ci.org/markatk/serial-unit-testing)
[![Build status](https://ci.appveyor.com/api/projects/status/7b9osucvqpqp9ebd/branch/master?svg=true)](https://ci.appveyor.com/project/markatk/serial-unit-testing/branch/master)

# Description

Serial unit testing is a cross-platform cli application and rust library. Using serial unit testing communication and testing with any serial port device can be done and most important automated.

# Application

The application is structured in the following sub commands:

- `list`: List all available serial ports
- `send`: Send data to a serial port
- `check`: Send data to a serial port and check for correct response
- `monitor`: Continously display serial port data
- `run`: Run a script on a serial port
- `verify`: Verify a script can be parsed without failure
- `help`: Print information about the application or a subcommand

# Script syntax

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

# License

MIT License

Copyright (c) 2019

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
