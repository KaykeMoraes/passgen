# passgen

`passgen` is a small command-line password generator written in Rust. It
creates pronounceable, hyphen-separated passwords and can optionally copy a
generated password to the system clipboard.

## Features

- Generates a 20-character password in the form `xxxxxx-xxxxxx-xxxxxx`
- Uses consonant/vowel patterns to make passwords easier to read
- Includes at least one uppercase letter and one digit
- Copies passwords directly to the clipboard with `--copy`
- Provides built-in help and version output

## Installation

### From source

Install [Rust](https://www.rust-lang.org/tools/install), then clone and build
the project:

```bash
git clone <repository-url>
cd password-generator
cargo install --path .
```

The installed executable is named `passgen`.

### Build without installing

```bash
cargo build --release
./target/release/passgen
```

## Usage

Generate a password and print it to standard output:

```bash
passgen
```

Example output:

```text
vexuQz-lomira-Tev7xa
```

Copy a generated password to the system clipboard:

```bash
passgen --copy
```

Short options are also available:

```bash
passgen -c
passgen -h
passgen -v
```

### Options

| Option | Description |
| --- | --- |
| `-c`, `--copy` | Generate a password and copy it to the clipboard |
| `-h`, `--help` | Display usage information |
| `-v`, `--version` | Display the application version |

Unknown options are reported as errors.

## Clipboard support

The `--copy` and `-c` options use the platform's native clipboard command:

- **Linux with Wayland:** [`wl-copy`](https://github.com/bugaevc/wl-clipboard)
- **Linux with X11:** [`xclip`](https://github.com/astrand/xclip)
- **macOS:** `pbcopy`, which is included with macOS
- **Windows:** PowerShell's `Set-Clipboard` command

On Linux, install the appropriate clipboard utility before using `--copy`.

## Development

Run the application directly with Cargo:

```bash
cargo run
cargo run -- --copy
```

Run the test suite:

```bash
cargo test
```

Create an optimized release build:

```bash
cargo build --release
```

## License

No license has been specified for this project yet.
