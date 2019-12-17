# Procespy: Process Monitoring tool in Rust
<p align="left">
  <a href="https://travis-ci.org/pankajchaudhary5/kode-combat-2019-procespy">
    <img alt="Build Status" src="https://travis-ci.org/PankajChaudhary5/kode-combat-2019-procespy.svg?branch=master">
  </a>
  <img alt="Stability stable" src="https://img.shields.io/badge/stability-stable-green.svg">
</p>
Procespy is a process Monitoring tool for Linux distributions, it helps the user to keep track of the running services in the system with their memory utilisation.
This tool will allow the user to set the threshold for a particular service in respect of memory utilisation. If any service will cross its threshold value then the process will terminate by providing a popup with a message that particular process is terminated and also sends an email to the user for providing the status of that service.

## Features of Procespy:
* Monitor memory utilised by each service.
* Allow users to set the threshold limit for a particular service.
* Terminate the process if any processes exceed its threshold limit.
* Send notification to the user.

We thrive for the best and want you to contribute towards a better Project. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for giving your valuable feedbacks and contributions.

## Setting up your environment

### Rustup.rs

Building this project requires [rustup](https://rustup.rs/), version 1.20.0 or more recent.
If you have an older version, run `rustup self update`.

To install on Windows, download and run [`rustup-init.exe`](https://win.rustup.rs/)
then follow the onscreen instructions.

To install on other systems, run:

```
curl https://sh.rustup.rs -sSf | sh
```

This will also download the current stable version of Rust, which this project won’t use.
To skip that step, run instead:

```
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain none
```
## Building

### Normal Build

```
git clone https://github.com/knoldus/kode-combat-2019-procespy.git
cd kode-combat-2019-procespy
cargo build
```
## License

Procespy is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE.md), and [LICENSE-MIT](LICENSE-MIT.md) for details.
