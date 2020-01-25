# Serial Unit Testing

[![Crates.io](https://img.shields.io/crates/v/serial-unit-testing.svg)](https://crates.io/crates/serial-unit-testing)
[![Documentation](https://docs.rs/serial-unit-testing/badge.svg)](https://docs.rs/crate/serial-unit-testing)
[![Build Status](https://travis-ci.org/markatk/serial-unit-testing.svg?branch=master)](https://travis-ci.org/markatk/serial-unit-testing)

# Description

Serial unit testing is a cross-platform cli application and rust library. Using serial unit testing communication and testing with any serial port device can be done and most important automated.

# Installation

If you have rust installed:

```
cargo install serial-unit-testing
```

If you don't have rust installed:

You can download the compiled binaries from the [releases](https://github.com/markatk/serial-unit-testing/releases). Downloads are available for Windows, macOS and Linux.

# Application

The application, called `sut` (which is short for **s**erial **u**nit **t**esting), is structured in the following sub commands:

- `list`: List all available serial ports
- `send`: Send data to a serial port
- `check`: Send data to a serial port and check for correct response
- `monitor`: Interactive serial communication monitor
- `run`: Run a script on a serial port
- `verify`: Verify a script can be parsed without failure
- `help`: Print information about the application or a sub command
- `version`: Print version information

# Test script language

The `run` and `verify` commands are used to work with the test script language to automate testing easily. For a complete syntax of the language see [script](doc/script.md).

```
# Example script

(Test One)              "h\n"  :  "Help"
(Test Two, repeat = 2) h"00FF" : h"00"
                       h"00af" : h"03"

[Group One]
(Group Test One) "gp\n" : "yes"
(Group Test Two) "gq\n" : "no"
```

# License

MIT License

Copyright (c) 2020 MarkAtk

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
