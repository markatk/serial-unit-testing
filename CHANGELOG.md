# Changelog

## [0.2.2] - 19.01.2020

### Added
- Add loopback serial port device. Can be used in every place a serial port is required (by opening device named "loopback")
- Add manual output scrolling in monitor command with shift + up/down keys. Current position and total lines will be displayed in bottom right corner
- Add crate features to be able to use the library without the binary dependencies and tests without colored dependency
- Add prelude module to simplify usage of the library

### Changes
- Fix newest output line not visible when scrolling in monitor
- Fix hexadecimal lower case response comparison in tests and check command

## [0.2.1] - 19.11.2019

### Added
- Add usb product name to list (can be disabled with raw option)

### Changes
- Improved script language documentation
- Move documentation to doc folder

## [0.2.0] - 12.11.2019

### Added
- Add pages to help window
- Add missing hot keys to help window
- Add cargo audit in travis
- Add input escaping to monitor

### Changes
- Change error handling to use custom error and result
- Make monitor helper functions available in library
- Fix error handling in serial

## [0.1.8] - 08.11.2019

### Added
- Add missing command line options to monitor
- Add missing ctrl hot keys to monitor

### Changes
- Improve window management in monitor

## [0.1.7] - 30.10.2019

### Added
- Add newline format to monitor
- Show port name and config in monitor title
- Add help window to monitor
- Add version command

### Changes
- Move windows CI testing to travis
- Fix octal and decimal format conversion on serial write
- Fix cursor position in monitor history

## [0.1.6] - 23.10.2019

### Added
- Add library documentation
- Add quiet option to run

### Changes
- Make monitor interactive: Add input field and text format options

## [0.1.5] - 24.02.2019

### Added
- Add various variations of read_str and check_str functions to serial

### Changes
- Improve radix_string conversion

## [0.1.4] - 21.02.2019

### Added
- Add cargo deploy to travis CI
- Add missing information to crate
- Add badges to crate

## [0.1.3] - 21.02.2019

### Added
- Add application description, script syntax and license to readme
- Add colors to run
- Add regex output matching in tests
- Add proper lexer and finite-state-machine for parsing
- Allow whitespaces in-between tokens
- Add various options to tests and test groups
- Add verbosity to run

### Changes
- Change crate version to 2018 edition
- Improve test result output
