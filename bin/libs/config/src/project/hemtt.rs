use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Features {
    #[serde(default)]
    config: hemtt_config::Options,

    #[serde(default)]
    dev: DevOptions,

    #[serde(default)]
    launch: LaunchOptions,

    #[serde(default)]
    build: BuildOptions,

    #[serde(default)]
    /// Can PBO prefixes have a leading slash?
    ///
    /// Default: false
    pbo_prefix_allow_leading_slash: Option<bool>,
}

impl Features {
    #[must_use]
    pub const fn config(&self) -> &hemtt_config::Options {
        &self.config
    }

    #[must_use]
    pub const fn dev(&self) -> &DevOptions {
        &self.dev
    }

    #[must_use]
    pub const fn launch(&self) -> &LaunchOptions {
        &self.launch
    }

    #[must_use]
    pub const fn build(&self) -> &BuildOptions {
        &self.build
    }

    #[must_use]
    pub const fn pbo_prefix_allow_leading_slash(&self) -> bool {
        if let Some(allow) = self.pbo_prefix_allow_leading_slash {
            allow
        } else {
            false
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DevOptions {
    #[serde(default)]
    exclude: Vec<String>,
}

impl DevOptions {
    #[must_use]
    pub fn exclude(&self) -> &[String] {
        &self.exclude
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LaunchOptions {
    #[serde(default)]
    /// Workshop mods that should be launched with the mod
    workshop: Vec<String>,

    #[serde(default)]
    // Extra launch parameters
    parameters: Vec<String>,
}

impl LaunchOptions {
    #[must_use]
    pub fn workshop(&self) -> &[String] {
        &self.workshop
    }

    #[must_use]
    pub fn parameters(&self) -> &[String] {
        &self.parameters
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BuildOptions {
    #[serde(default)]
    /// Should optionals be built into their own mod?
    /// Default: true
    optional_mod_folders: Option<bool>,
}

impl BuildOptions {
    #[must_use]
    pub const fn optional_mod_folders(&self) -> bool {
        if let Some(optional) = self.optional_mod_folders {
            optional
        } else {
            true
        }
    }
}
