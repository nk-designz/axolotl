use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, Debug, PartialEq)]
pub enum MediaType {
    SchemaV1,
    SchemaV2,
    ManifestList,
    ContainerConfig,
    Layer,
    LayerForeign,
    PluginConfig,
}

impl ToString for MediaType {
    fn to_string(&self) -> String {
        match self {
            MediaType::SchemaV1 => "application/vnd.docker.distribution.manifest.v1+json",
            MediaType::SchemaV2 => "application/vnd.docker.distribution.manifest.v2+json",
            MediaType::ManifestList => "application/vnd.docker.distribution.manifest.list.v2+json",
            MediaType::ContainerConfig => "application/vnd.docker.container.image.v1+json",
            MediaType::Layer => "application/vnd.docker.image.rootfs.diff.tar.gzip",
            MediaType::LayerForeign => "application/vnd.docker.image.rootfs.foreign.diff.tar.gzip",
            MediaType::PluginConfig => "application/vnd.docker.plugin.v1+json",
        }
        .to_string()
    }
}

impl Serialize for MediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

pub struct MediaTypeError;

impl fmt::Display for MediaTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!")
    }
}

impl fmt::Debug for MediaTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl FromStr for MediaType {
    type Err = MediaTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "application/vnd.docker.distribution.manifest.v1+json" => Ok(MediaType::SchemaV1),
            "application/vnd.docker.distribution.manifest.v2+json" => Ok(MediaType::SchemaV2),
            "application/vnd.docker.distribution.manifest.list.v2+json" => {
                Ok(MediaType::ManifestList)
            }
            "application/vnd.docker.container.image.v1+json" => Ok(MediaType::ContainerConfig),
            "application/vnd.docker.image.rootfs.diff.tar.gzip" => Ok(MediaType::Layer),
            "application/vnd.docker.image.rootfs.foreign.diff.tar.gzip" => {
                Ok(MediaType::LayerForeign)
            }
            "application/vnd.docker.plugin.v1+json" => Ok(MediaType::PluginConfig),
            _ => Err(MediaTypeError),
        }
    }
}

struct MediaTypeVisitor;

impl<'de> Visitor<'de> for MediaTypeVisitor {
    type Value = MediaType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid docker media type")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match MediaType::from_str(value) {
            Ok(mt) => Ok(mt),
            Err(err) => Err(E::custom(format!(
                "Error: {0}; {1} is not a valid mediatype!",
                err, value
            ))),
        }
    }
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_str())
    }
}

impl<'de> Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<MediaType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(MediaTypeVisitor)
    }
}
