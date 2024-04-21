use std::collections::HashMap;

use semver::{VersionReq, Version};

pub const MANIFEST_FILE: &str = "project.meteor";
// pub const LOCK_FILE: &str = "meteor.lock";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestConfig {
    pub name: String,
    pub version: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<String>,
    dependencies: HashMap<String, ManifestVersion>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum ManifestVersion {
    Version(VersionReq),
    Git { git: String, commit: String },
    Path { path: String },
}

impl ManifestConfig {
    pub fn create(name: String) -> ManifestConfig {
        ManifestConfig {
            name,
            version: Version::parse("0.1.0").unwrap(),
            description: None,
            readme: None,
            repo: None,
            dependencies: HashMap::new(),
        }
    }

    // pub fn from_string(contents: String) -> ManifestConfig {
    //     hcl::from_str(&contents).unwrap()
    // }

    pub fn to_string(&self) -> String {
        hcl::to_string(self).unwrap()
    }

    // pub fn add_dependency(&mut self, name: String, version: ManifestVersion) {
    //     self.dependencies.insert(name, version);
    // }
}
