use i18n_again::format_t;
use i18n_again::locale;

// Init translations for current crate.
i18n_again::i18n!("examples/app/locales");

fn main() {}

#[test]
fn test_example_app() {
    i18n_again::set_locale("en");
    assert_eq!(
        format_t!("hello", name = "Longbridge"),
        "Hello, Longbridge!"
    );
    assert_eq!(format_t!("view.buttons.ok"), "Ok");
    assert_eq!(format_t!("view.buttons.cancel"), "Cancel");
    assert_eq!(
        format_t!("view.datetime.about_x_hours", count = "10"),
        "about 10 hours"
    );

    assert_eq!(
        format_t!("hello", locale = "fr", name = "Longbridge"),
        "Bonjour, Longbridge!"
    );
    i18n_again::set_locale("fr");
    assert_eq!(
        format_t!("hello", name = "Longbridge"),
        "Bonjour, Longbridge!"
    );
    assert_eq!(
        format_t!("view.datetime.about_x_hours", count = "10"),
        "environ 10 heures"
    );
}
