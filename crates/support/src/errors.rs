#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Failed to serialize / deserialize")]
    SerDe,

    #[error(transparent)]
    Postcard(#[from] postcard::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),

    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error("Missing metadata in manifest file {}", .0.display())]
    ManifestMissingMetadata(std::path::PathBuf),

    #[error("Must be package, workspace is not supported {}", .0.display())]
    ManifestNotPackage(std::path::PathBuf),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
