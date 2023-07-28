use anyhow::Context;
use rust_embed::RustEmbed;
use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
};

#[derive(Debug, Clone, Copy)]
pub enum ProjectTemplate {
    Apim,
    Migw,
    Mi,
}

#[derive(Debug, Clone, Copy)]
pub enum Flavor {
    Docker,
}

#[derive(RustEmbed)]
#[folder = "fragments"]
#[allow(clippy::upper_case_acronyms)]
struct FRAGMENTS;

impl Display for ProjectTemplate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Apim => write!(f, "API Manager"),
            Self::Mi => write!(f, "Micro Integrator"),
            Self::Migw => write!(f, "Microgateway"),
        }
    }
}

impl Flavor {
    pub const fn slect_text<'a>(&self) -> &'a str {
        match self {
            Self::Docker => "Docker",
        }
    }
}

impl Display for Flavor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Docker => write!(f, "Docker"),
        }
    }
}

impl ProjectTemplate {
    pub const ALL: [Self; 3] = [Self::Apim, Self::Mi, Self::Migw];

    pub const fn flavors(&self) -> &[Flavor] {
        match self {
            Self::Apim => &[Flavor::Docker],
            Self::Mi => &[Flavor::Docker],
            Self::Migw => &[Flavor::Docker],
        }
    }

    pub fn render(&self, _target_dir: &std::path::Path, flavor: Flavor) -> anyhow::Result<()> {
        let manifest_bytes = FRAGMENTS::get(&format!("fragment-{self}/pom.xml"))
            .with_context(|| "Failed to get manifest bytes")?
            .data;

        let manifest_str = String::from_utf8(manifest_bytes.to_vec())?;

        let files = FRAGMENTS::iter()
            .filter(|f| PathBuf::from(f.to_string()).starts_with(&format!("fragment-{self}/")))
            .map(|f| f.to_string())
            .collect::<Vec<_>>();

        println!("{:?}", files);

        Ok(())
    }
}
