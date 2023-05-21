//! Pass on an environment variable the contains all translations in serialized single file.
//!
//! The content is an accumulated lookup table file, which allows easier handling on the `format_t` side.

use super::*;

use serde::Deserialize;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct I18n {
    load_path: std::path::PathBuf,
    available_locales: Vec<String>,
    default_locale: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Metadata {
    i18n: I18n,
}

pub fn prepare_from_manifest() -> Result<()> {
    let dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    prepare(dir)?;
    Ok(())
}

pub fn prepare(manifest_dir: impl AsRef<std::path::Path>) -> Result<()> {
    let manifest_dir = manifest_dir.as_ref();
    let manifest_path = manifest_dir.join("Cargo.toml");
    let manifest_path = &manifest_path;
    println!(
        "cargo:warning=Using manifest file: {}",
        manifest_path.display()
    );

    let manifest = fs_err::read_to_string(manifest_path)?;
    let manifest: cargo_toml::Manifest<Metadata> = toml::from_str(&manifest)?;
    let pkg = manifest
        .package
        .as_ref()
        .ok_or_else(|| Error::ManifestNotPackage(manifest_path.to_owned()))?;
    let name = pkg.name();
    let metadata = pkg
        .metadata
        .as_ref()
        .ok_or_else(|| Error::ManifestMissingMetadata(manifest_path.to_owned()))?;
    let i18n = &metadata.i18n;

    let locale_dir = manifest_dir.join(&i18n.load_path);

    let serialized = fs_err::canonicalize(&locale_dir)?.join(".i18n-serialize.postcard");

    println!("cargo:rerun-if-changed={}", serialized.display());

    for lang in i18n.available_locales.iter() {
        println!(
            "cargo:rerun-if-changed={}",
            locale_dir.join(format!("{}.{}.yml", name, lang)).display()
        );
    }

    println!(
        "cargo:rustc-env=I18N_SERIALIZED_TRANSLATIONS={}",
        serialized.display()
    );

    combine_and_serialize(&locale_dir, &serialized)?;

    Ok(())
}

/// Init I18n translations.
///
/// This will load all translations by glob `**/*.yml` from the given path and prepare a file to be included in the compiled proc macro.
pub fn combine_and_serialize(locale_dir: &Path, dest: &Path) -> Result<()> {
    let translations = locales_yaml_files_to_translation_map(&locale_dir)?;

    let serialized = self::serialize(translations)?;
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(dest)?;
    f.write_all(serialized.as_slice())?;
    Ok(())
}
