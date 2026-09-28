# ftproto

Console FTP server written in Rust using only std. <br>
No async runtime, no dependencies.<br>
Thread-per-connection with std::thread and mpsc events. <br>
Model-view-controller layout with one function per file.<br>

## Features

- FTP command set: USER, PASS, QUIT, NOOP, SYST, FEAT,
  OPTS, TYPE, MODE, STRU, ALLO, PWD, CWD, CDUP, PASV,
  EPSV, PORT, LIST, NLST, SIZE, MDTM, MKD, RMD, DELE,
  RNFR, RNTO, REST, RETR, STOR, APPE, ABOR
- passive and active data connections
- multiple simultaneous connections with the same user
- read, write, delete, rename, mkdir, rmdir
- clients jailed inside the files root; `..` clamps
  at the root, drive-letter style parts rejected
- event log shown only on demand, capped at 1000 lines

## Requirements

- Rust 2021 edition
- a terminal with ANSI escape support (for `k`)

## Build

```sh
cargo build --release
```

Binary lands at `target/release/ftproto`.

## Run

```sh
cargo run
```

## Menu

```text
a -- start server
b -- stop server
c -- setup server
s -- show status
m -- show menu
k -- clear screen
x -- exit
```

`a`, `b`, `c`, `s` run on worker threads; the
menu keeps accepting input while the server runs.

## Default configuration

| setting     | value    |
| ----------- | -------- |
| listen ip   | 0.0.0.0  |
| listen port | 2121     |
| files root  | files/   |
| user        | ftproto  |
| password    | ftproto  |

The files root is created on server start if missing.

## Setup

Press `c`. Prompts in order: listen port, listen ip,
files folder. Blank input keeps the current value; the
prompt shows the current value in brackets. On the final
confirmation (`y`) with the server running, all
connections are closed, the server stops, settings are
applied, and the server restarts.

## Connecting

netkit ftp:

```sh
ftp 127.0.0.1 2121
```

url form with credentials:

```sh
ftp ftp://ftproto:ftproto@127.0.0.1:2121/
```

curl:

```sh
curl -l ftp://ftproto:ftproto@127.0.0.1:2121/
curl -T file ftp://ftproto:ftproto@127.0.0.1:2121/
curl -o out ftp://ftproto:ftproto@127.0.0.1:2121/file
```

FileZilla: host `127.0.0.1`, port `2121`, user
`ftproto`, pass `ftproto`.

Password-free ftp logins via `~/.netrc`:

```text
machine 127.0.0.1
login ftproto
password ftproto
```

## Status and events

Server events (connections, logins, transfers,
lifecycle) accumulate in a vector and print only when
`s` is entered. Nothing streams to the console during
operation.

## Tests

```sh
cargo test
```

Unit tests live in `tests/main.rs` and cover config
defaults, virtual path clamping, command parsing, path
resolution, list flag stripping, passive ip encoding,
date formatting, and setup flow validation.

## Layout

```text
Cargo.toml
README.md
src/
  main.rs
  lib.rs
  app/
    mod.rs
    model/
      mod.rs
      config.rs
      events.rs
      runtime.rs
      setup.rs
      state.rs
    view/
      mod.rs
      menu.rs
      setup_prompt.rs
      status.rs
    control/
      mod.rs
      accept.rs
      apply.rs
      basics.rs
      commands.rs
      dataconn.rs
      dispatch.rs
      fsops.rs
      ftp_time.rs
      ftp_util.rs
      listing.rs
      rename.rs
      server.rs
      session.rs
      session_io.rs
      setup_advance.rs
      setup_flow.rs
      stop.rs
      store.rs
      stream_io.rs
      stream_util.rs
      transfer.rs
tests/
  main.rs
```

Every source file stays under 100 lines. `mod.rs`
files declare and re-export child modules and hold type
aliases.

## Constraints

- no TLS; credentials cross the wire in cleartext
- TYPE command accepted but transfers are always binary
- listen ip is an ip address, not a hostname
- large single-file listings write in one pass; a slow
  consumer can hit the 30 second data timeout


## License

This project is licensed under the BSD 3-Clause License - see the LICENSE
file for details.

Copyright (c) 2026 alexander14k28@gmail.com

See [LICENSE](LICENSE) for the license governing this project.
