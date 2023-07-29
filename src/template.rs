use anyhow::Context;
use rust_embed::RustEmbed;
use std::{
    fmt::{self, Display, Formatter},
    fs,
    path::{self, PathBuf},
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

    pub fn render(&self, target_dir: &std::path::Path, flavor: Flavor) -> anyhow::Result<()> {
        let manifest_bytes = FRAGMENTS::get(&format!("fragment-{self}/pom.xml"))
            .with_context(|| "Failed to get manifest bytes")?
            .data;

        let manifest_str = String::from_utf8(manifest_bytes.to_vec())?;

        //render all files that do not need custom rendering

        // TODO: create project template file that contains all the files that need to be custom
        // rendered -> create array of those files -> filter out those files from the list of all

        let files = FRAGMENTS::iter()
            .filter(|f| {
                f.to_string().starts_with(&format!("fragment-{self}/"))
                    && !f.to_string().ends_with("pom.xml")
            })
            .map(|f| f.to_string())
            .collect::<Vec<_>>();

        for file in files {
            let data = FRAGMENTS::get(&file)
                .with_context(|| format!("Failed to get file {}", file))?
                .data;

            // remove the first component, which is certainly the fragment directory they were in before getting embeded into the binary
            let p = path::PathBuf::from(file)
                .components()
                .skip(1)
                .collect::<path::PathBuf>();

            let p = target_dir.join(p);
            let file_name = p.file_name().unwrap();

            let parent = p.parent().unwrap();
            fs::create_dir_all(parent)?;
            fs::write(parent.join(file_name), &data)?;
        }

        Ok(())
    }
}
