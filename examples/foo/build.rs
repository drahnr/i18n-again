fn main() -> Result<(), Box<dyn std::error::Error>> {
    i18n_again_support::prepare_from_manifest()?;
    Ok(())
}
