//! [![CI](https://github.com/drahnr/i18n-again/actions/workflows/ci.yml/badge.svg)](https://github.com/drahnr/i18n-again/actions/workflows/ci.yml) [![Docs](https://docs.rs/i18n-again/badge.svg)](https://docs.rs/i18n-again/) [![Crates.io](https://img.shields.io/crates/v/i18n-again.svg)](https://crates.io/crates/i18n-again)
//!
//! Rust I18n is use Rust codegen for load YAML file storage translations on compile time, and give you a format_t! macro for simply get translation texts.
//!
//! > Inspired by [ruby-i18n](https://github.com/ruby-i18n/i18n).
//!
//! ### Usage
//! Add crate dependencies in your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! once_cell = "1.10.0"
//! i18n-again = "0"
//! ```
//!
//! Load macro and init translations in `lib.rs`
//!
//! ```ignore
//! // Load I18n macro, for allow you use `format_t!` macro in anywhere.
//! #[macro_use]
//! extern crate i18n_again;
//!
//! // Init translations for current crate.
//! i18n!("locales");
//! ```
//!
//! You must put I18n YAML files in `locales/` folder.
//!
//! ```bash
//! locales/
//! ├── en.yml
//! ├── zh-CN.yml
//! ```
//!
//! For example of `en.yml`:
//!
//! ```yml
//! en:
//!   hello: Hello world
//!   messages:
//!     hello: Hello, %{name}
//! ```
//!
//! Now you can use `format_t!` macro in anywhere.
//!
//! ```ignore
//! format_t!("hello");
//! // => "Hello world"
//!
//! format_t!("hello", locale = "zh-CN);
//! // => "你好世界"
//!
//! format_t!("messages.hello", name = "world");
//! // => "Hello, world"
//!
//! format_t!("messages.hello", locale = "zh-CN", name = "Jason");
//! // => "你好, Jason"
//! ```
//!
//! You can use `i18n_again::set_locale` to change the current locale in runtime.
//!
//! ```rs
//! i18n_again::set_locale("zh-CN");
//! i18n_again::locale();
//! // => "zh-CN"
//! ```
//!
//!

// include!(concat!(env!("OUT_DIR"), "/i18n.rs"));
use once_cell::sync::Lazy;
use std::sync::Mutex;

mod error;
pub use error::{Error, Result};

/// Format using an i18n translated format string.
///
/// ```ignore
/// // Simple get text with current locale
/// format_t!("greeting"); // greeting: "Hello world" => "Hello world"
/// // Get a special locale's text
/// format_t!("greeting", locale = "de"); // greeting: "Hallo Welt!" => "Hallo Welt!"
///
/// // With variables
/// format_t!("messages.hello", "world"); // messages.hello: "Hello, {}" => "Hello, world"
/// format_t!("messages.foo", "Foo", "Bar"); // messages.foo: "Hello, {} and {}" => "Hello, Foo and Bar"
///
/// // With locale and variables
/// format_t!("messages.hello", locale = "de", "Jason"); // messages.hello: "Hallo, {}" => "Hallo, Jason"
/// ```
pub use i18n_again_macro::format_t;

static CURRENT_LOCALE: Lazy<Mutex<&'static str>> = Lazy::new(|| Mutex::new("en"));

pub fn set_locale(locale: &str) {
    let mut current_locale = CURRENT_LOCALE.lock().unwrap();
    *current_locale = Box::leak::<'static>(Box::new(locale.to_owned()));
}

pub fn locale() -> &'static str {
    &CURRENT_LOCALE.lock().unwrap()
}

#[macro_export]
macro_rules! i18n {
    ($path:literal) => {
        // compile_error!("i18n! macro is deprecated / not yet implemented")
    };
}
