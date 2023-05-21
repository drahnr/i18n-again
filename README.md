# Rust I18n

⚠️ E X P E R I M E N T A L ⚠️

[![Docs](https://docs.rs/i18n-again/badge.svg)](https://docs.rs/i18n-again/) [![Crates.io](https://img.shields.io/crates/v/i18n-again.svg)](https://crates.io/crates/i18n-again)

Orignally forked from [rust-i18n](https://github.com/longbridgeapp/rust-i18n) but taking a different approach on injecting.

`format_t!` is the one and only entry point and at the same time, having all translations kept inside the binary and selected at runtime with
a simple `match` statement. 

TODO:

* [ ] check with multiple crates that use this crate
* [ ] make tests that use `format_t!` work, and not fail any file lookups
* [ ] `proc-macro` efficiency is far from good

## Features

- Codegen on compile time for includes translations into binary.
- Global `format_t!` macro for loading localized text in everywhere.
- Use YAML for mapping localized text, and support mutiple YAML files merging.
- `cargo i18n` Command line tool for checking and extract untranslated texts into YAML files.

## Installation

Rust I18n also provided a `cargo i18n` command line tool help you process translations.

```bash
cargo install i18n-again
```

## Usage

Add crate dependencies in your Cargo.toml and setup I18n config:

```toml
[dependencies]
once_cell = "1.10.0"
i18n-again = "0"

[package.metadata.i18n]
# The available locales for your application, default: ["en"].
available-locales = ["en", "zh-CN"]

# The default locale, default: "en".
default-locale = "en"

# Path for your translations YAML file, default: "locales".
load-path = "locales"
```

Load macro and init translations in `build.rs`

```rs
//! build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    i18n_again_support::prepare_from_manifest()?;
    Ok(())
}
```

Or you can import by use directly:

```rs
//! src/lib.rs or src/main.rs
use i18n_again::format_t;

fn main() {
    i18n_again::set_locale("de_DE");
    println!("{}", format_t!("hello"));
}
```

Make sure all YAML files (containing the localized mappings) are located in the `locales/` folder of the project root directory:

```
.
├── Cargo.lock
├── Cargo.toml
├── locales
│   ├── en.yml
│   ├── de_DE.yml
│   └── klingonean.yml
└── src
    └── main.rs
```

In the YAML files, specify the localization keys and their corresponding values, for example, in `en.yml`:

```yml
en: # The language code of this mapping file
  hello: Hello world # A simple key -> value mapping
  messages:
    hello: Hello, %{name} # A nested key.sub_key -> value mapping, in this case "messages.hello" maps to "Hello, %{name}"
```

And example of the `zh-CN.yml`:

```yml
zh-CN:
  hello: 你好世界
  messages:
    hello: 你好, %{name}
```

### Loading Localized Strings in Rust

Import the `format_t!` macro from this crate into your current scope:

```rs
use i18n_again::format_t;
```

Then, simply use it wherever a localized string is needed:

```rs
format_t!("hello");
// => "Hello world"

format_t!("hello", locale = "zh-CN");
// => "你好世界"

format_t!("messages.hello", name = "world");
// => "Hello, world"

format_t!("messages.hello", locale = "zh-CN", name = "Jason");
// => "你好, Jason"
```

### Setting and Getting the Global Locale

You can use `i18n_again::set_locale` to set the global locale at runtime, so that you don't have to specify the locale on each `format_t!` invocation.

```rs
i18n_again::set_locale("zh-CN");

let locale = i18n_again::locale();
assert_eq!(locale, "zh-CN");
```

### Extract the untranslated texts

Rust I18n providered a `i18n` bin for help you extract the untranslated texts from the source code and then write into YAML file.

```bash
$ cargo install i18n-again
# Now you have `cargo i18n` command
```

After that the untranslated texts will be extracted and saved into `locales/TODO.en.yml` file.

You also can special the locale by use `--locale` option:

```bash
$ cd your_project_root_directory
$ cargo i18n

Checking [en] and generating untranslated texts...
Found 1 new texts need to translate.
----------------------------------------
Writing to TODO.en.yml

Checking [fr] and generating untranslated texts...
Found 11 new texts need to translate.
----------------------------------------
Writing to TODO.fr.yml

Checking [zh-CN] and generating untranslated texts...
All thing done.

Checking [zh-HK] and generating untranslated texts...
Found 11 new texts need to translate.
----------------------------------------
Writing to TODO.zh-HK.yml
```

Run `cargo i18n -h` to see details.

```bash
$ cargo i18n -h
cargo-i18n 0.5.0
---------------------------------------
Rust I18n command for help you simply to extract all untranslated texts from soruce code.

It will iter all Rust files in and extract all untranslated texts that used `format_t!` macro.
And then generate a YAML file and merge for existing texts.

https://github.com/drahnr/i18n-again

USAGE:
    cargo i18n [OPTIONS] [--] [source]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <source>    Path of your Rust crate root [default: ./]
```

## Debugging the Codegen Process

The `RUST_I18N_DEBUG` environment variable can be used to print out some debugging infos when code is being generated at compile time.

```bash
$ RUST_I18N_DEBUG=1 cargo build
```

## Example

A minimal example of using i18n-again can be found [here](https://github.com/drahnr/i18n-again/tree/main/examples).

## License

MIT
