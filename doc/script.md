# Serial-Unit-Testing Scripts

One of the key features of the serial-unit-testing application/framework is to automate testing serial devices. This can be done with the serial-unit-testing 
script language. In this scripts test can be defined, grouped and executed very quickly. In the following the script syntax is explained.

## Example script

```
# Example script

(Test One)              "h\n"  :  "Help"
(Test Two, repeat = 2) h"00FF" : h"00"
                       h"00af" : h"03"

[Group One]
(Group Test One) "gp\n" : "yes"
(Group Test Two) "gq\n" : "no"
```

## Execution

Script can be run with the `run` command. To verify a script for correct syntax the `verify` command can be used.

In addition the serial-unit-testing library provides access to the `parser::parse_file` to handle testing in an own application.

## Syntax

Each (non empty) line represents a test, test group or comment. Lines are terminated with the newline character `\n`.

### Comment

Each line starting with the hash sign `#` will be treated as a comment and ignored by the parser.

Example: `# This is a comment`

## Group

Test can be grouped to separate them visually and optionally add group settings to them. A group line starts with `[` followed by the group name and ends 
with `]`. All tests following the group line will belong to this group as long as no other group is introduced. If no group is present the tests will be 
independent.

Example: `[Group One]`

Additionally groups can have settings which are applied to all tests in the group. Group settings overwrite execution settings (which are set by the 
command line, for more information see the `run` command) and in turn are overwritten by the individual test settings. Settings start with the setting name, 
followed by the equal sign `=` and the setting value. Multiple settings are separated by comma `,`. The first setting must be prefixed with a comma to separate
from the group name. Whitespaces before and after the equal sign and comma will be ignored.

Example: `[Group Two, ignore-case = true, delay = 1s]`

## Test

Each line not being a comment or group (and not being empty) is interpreted as a test. Tests have an input and an output part which both must be present and are 
separated with a colon `:`. Tests may start with a name surrounded with brackets `()`, otherwise the input content is used as the test's display name. 
The input/output content must be surrounded with quotation marks `""`. Optionally the content can be prefixed with one of the following format specifier, 
otherwise text mode is used. Input and output format specifiers can be different.

 - `b`: Binary mode
 - `o`: Octal mode
 - `d`: Decimal mode
 - `h`: Hexadecimal mode

Content in text mode can be escaped with a backslash `\` thus characters like `\n` can be included.

Example: `(Test One)h"58990d" : "OK\r"`

Additionally tests can have settings which will overwrite possible group or execution settings. Test settings have the same syntax like group settings:
They start with the setting name, followed by the equal sign `=` and the setting value. Multiple settings are separated by comma `,`. The first setting must be 
prefixed with a comma to separate from the group name. Whitespaces before and after the equal sign and comma will be ignored.

Example: `(Test Two, repeat = 10) "Result" : "OK"`

## Settings

Following settings can be set for groups or individual tests:

- **ignore_case**: When set the casing will be ignored in output comparison.
- **repeat**: The test(s) will be repeated the given amount. Only if all repeats match the result will be good.
- **delay**: Wait given amount in milliseconds before executing the test. If used in conjunction with **repeat** the delay will be waited before every repeat.
- **timeout**: Wait the given duration in milliseconds before the test will fail with a timeout.
- **allow_failure**: If set the test is allowed to fail.
- **verbose**: Print additional information when executing the test.

Following settings can be additionally set for groups (and are not valid for tests):

- **stop_on_failure**: If set the group will stop on the first test failing. This will not stop other groups from running.
- **disabled**: If set the group will not be run.
