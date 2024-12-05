# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/smedegaard/hip-rs/compare/v0.1.1...v0.1.2) - 2024-12-05

### Other

- closes 112. add core::stream ([#128](https://github.com/smedegaard/hip-rs/pull/128))
- rename runtime -> core ([#127](https://github.com/smedegaard/hip-rs/pull/127))

## [0.1.1](https://github.com/smedegaard/hip-rs/compare/v0.1.0...v0.1.1) - 2024-12-02

### Other

- Actions ([#124](https://github.com/smedegaard/hip-rs/pull/124))
- Actions ([#123](https://github.com/smedegaard/hip-rs/pull/123))
- Actions ([#122](https://github.com/smedegaard/hip-rs/pull/122))
- Actions ([#121](https://github.com/smedegaard/hip-rs/pull/121))
- Actions ([#120](https://github.com/smedegaard/hip-rs/pull/120))
- run on updated release ([#119](https://github.com/smedegaard/hip-rs/pull/119))
- Actions ([#118](https://github.com/smedegaard/hip-rs/pull/118))
- update readme.md ([#117](https://github.com/smedegaard/hip-rs/pull/117))
- Actions ([#116](https://github.com/smedegaard/hip-rs/pull/116))
- Actions ([#115](https://github.com/smedegaard/hip-rs/pull/115))
- add github action ([#114](https://github.com/smedegaard/hip-rs/pull/114))
- closes 84. hipmemset ([#113](https://github.com/smedegaard/hip-rs/pull/113))
- closes 59. hipmemcpy ([#110](https://github.com/smedegaard/hip-rs/pull/110))
- closes 49. hipextmallocwithflags ([#107](https://github.com/smedegaard/hip-rs/pull/107))
- closes 105. cleanup MemoryPointer ([#106](https://github.com/smedegaard/hip-rs/pull/106))
- closes 21. hipsetdevice ([#104](https://github.com/smedegaard/hip-rs/pull/104))
- closes 48. hipmalloc ([#101](https://github.com/smedegaard/hip-rs/pull/101))
- closes [#99](https://github.com/smedegaard/hip-rs/pull/99). 'runtime/init.rs imports HipErrorKind' ([#100](https://github.com/smedegaard/hip-rs/pull/100))
- project restructure
- closes [#6](https://github.com/smedegaard/hip-rs/pull/6). hipdevicegetbypcibusid ([#18](https://github.com/smedegaard/hip-rs/pull/18))
- closes [#5](https://github.com/smedegaard/hip-rs/pull/5). Adds hipdevicegetpcibusid() ([#17](https://github.com/smedegaard/hip-rs/pull/17))
- closes [#4](https://github.com/smedegaard/hip-rs/pull/4). hipdevicegetp2pattribute ([#16](https://github.com/smedegaard/hip-rs/pull/16))
- closes [#13](https://github.com/smedegaard/hip-rs/pull/13). 'revert inititaliz() to original. get_device_uuid_bytes is now private' ([#15](https://github.com/smedegaard/hip-rs/pull/15))
- closes [#3](https://github.com/smedegaard/hip-rs/pull/3) hipdevicegetuuid ([#14](https://github.com/smedegaard/hip-rs/pull/14))
- 2 hipdevicegetname ([#12](https://github.com/smedegaard/hip-rs/pull/12))
- closes [#1](https://github.com/smedegaard/hip-rs/pull/1).  add runtime_get_version() ([#9](https://github.com/smedegaard/hip-rs/pull/9))
- 7 hipdevicetotalmem ([#8](https://github.com/smedegaard/hip-rs/pull/8))
- 'use .to_result(), Luke!'
- 'add semver crate. get_device_compute_capability() returns Result<Version>'
- 'update Version constructor'
- 'add Version type. Add get_device_compute_capability()'
- remove initialize error test
- 'print in test_initialize_error()'
- 'experiments with initialize()'
- 'set_device() returns Result<Device>'
- 'use from_kind()'
- 'update HipError constructor'
- 'initialize() catches panic'
- 'update init tests'
- 'remove test mock'
- 'update all tests'
- 'update tests'
- 'WIP'
- 'types'
- 'i32'
- 'get_device. back to original return'
- 'get_device. update signature'
- :get_device(). declare device_id as -1
- 'update runtime::get_device()'
- use result and error types
- '.expect() consumes result'
- 'test_initialization'
- 'use HipErrorKind'
- 'hip error codes to u32'
- 'add error and result types'
- 'remove test'
- 'add test_device_count_without_init'
- 'fix path to runtime wrapper'
- 'restructure project'
