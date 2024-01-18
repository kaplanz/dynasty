<div class="oranda-hide">

# dynasty

[![Rust][ci.rust.badge]][ci.rust.hyper]
[![dependency status][deps.badge]][deps.hyper]

</div>

Dynasty is a dynamic DNS client written in Rust and designed to be easily
extensible to support any DNS provider service. It currently only supports
[Cloudflare][cloudflare], but please make a pull request if you'd like to add
support for another service!

## Install

### Releases

Whenever Dynasty publishes a new release, pre-compiled binaries will be made
available for download on the [releases page][releases].

### Source

To install Dynasty from source, install Rust (with Cargo), clone this
repository, and build with:

```shell
cargo build --release
```

To install, move the compiled executable somewhere in your path:

```shell
# This should work on most *nix systems
mv target/release/dynasty /usr/local/bin
```

## Configuration

Before using Dynasty, you must specify your DNS provider and domains by creating
a configuration file called `config.toml`. Dynasty looks for this configuration
file according the platform-specific location, such as `~/.config/dynasty/` on
Linux. See [here][dirs] for details relating to your platform.

A sample configuration file would look something like this:

```toml
resolver = "dig @resolver4.opendns.com myip.opendns.com +short"

[daemon]
timeout  = "24h"

[[services]]
provider = "Cloudflare"
token    = "fkdzsjxnfi345wfni5dnfcdkncka4_dw4n44f_ce"
zone     = "0123456789abcdef0123456789abcdef"
record   = "fedcba9876543210fedcba9876543210"
```

### Resolver

In order to resolve your server's public IP, a resolver command must be supplied
to Dynasty. Unless you know what you're doing, I recommend leaving it blank to
use the default resolver.

### Daemon

Specify how Dynasty should run in [daemon mode](#daemon-mode).

### Services

Services can be configured using array entries to the `services` table, with
each entry representing a distinct service-(sub)domain pair. Note that providers
may require different configuration options.

## Usage

### Command-line options

```
Dynamic DNS Client

Usage: dynasty [OPTIONS]

Options:
  -c, --conf <CONF>  Configuration file [default: "~/.config/dynasty/config.toml"]
  -d, --daemon       Run as a daemon
  -n, --dry-run      Perform a dry run
  -v, --verbose...   More output per occurrence
  -q, --quiet...     Less output per occurrence
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version
```

### Daemon mode

By default, when run Dynasty will refresh all configured DNS entries once. If
you want to run Dynasty as a daemon, you can do so by specifying the `-d` or
`--daemon` flag on the command-line. Before doing so, you must configure the
`timeout` parameter in the configuration, which specifies how often Dynasty
will check and refresh your DNS entries.

It is recommended to use an [init] service (such as [runit], or [systemd]) to
automatically run Dynasty on startup if you're using it in daemon mode.

## Alternatives

- [ddclient]: A well-known Perl DDNS client, although now unmaintained.
- [inadyn]:   Another open-source DDNS client, written in C.

<!-- Reference-style links -->
[cloudflare]: https://www.cloudflare.com
[ddclient]:   https://ddclient.net
[dirs]:       https://docs.rs/dirs/latest/dirs/fn.config_dir.html
[inadyn]:     https://github.com/troglobit/inadyn
[init]:       https://en.wikipedia.org/wiki/Init
[releases]:   https://github.com/kaplanz/dynasty/releases
[runit]:      http://smarden.org/runit/
[systemd]:    https://systemd.io

<!-- Reference-style badges -->
[ci.rust.badge]: /../../actions/workflows/rust.yml/badge.svg
[ci.rust.hyper]: /../../actions/workflows/rust.yml
[deps.badge]:    https://deps.rs/repo/github/kaplanz/dynasty/status.svg
[deps.hyper]:    https://deps.rs/repo/github/kaplanz/dynasty
