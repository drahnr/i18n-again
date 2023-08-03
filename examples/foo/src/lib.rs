use i18n_again::format_t;

pub fn f() -> String {
    format_t!("hello {a} {b}", a="1", b="2")
}
