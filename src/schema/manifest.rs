use super::media_type::MediaType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManifestList {
    #[serde(rename = "schemaVersion")]
    pub schema_version: i8,
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
    pub manifests: Vec<Manifest>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manifest {
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
    pub size: i64,
    pub digest: String,
    pub platform: Platform,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Platform {
    pub architecture: String,
    pub os: String,
    #[serde(rename = "os.version")]
    pub os_version: String,
    #[serde(rename = "os.features")]
    pub os_features: Vec<String>,
    pub variant: String,
    pub features: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageManifests {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
    pub config: ImageManifetConfig,
    pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageManifetConfig {
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
    pub size: i64,
    pub digest: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Layer {
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
    pub size: i64,
    pub digest: String,
    pub urls: Vec<String>,
}
