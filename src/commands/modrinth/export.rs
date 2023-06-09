use modman::{utils::success, Error, Profile};
use rayon::prelude::*;
use serde::Serialize;
use std::{fs::File, io::Write};
use zip::{write::FileOptions, ZipWriter};

#[derive(Serialize)]
struct ModEnvironment {
    client: String,
    server: String,
}

#[derive(Serialize)]
struct FileHashes {
    sha1: String,
    sha512: String,
}

#[derive(Serialize)]
struct ModFile {
    path: String,
    hashes: FileHashes,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<ModEnvironment>,
    downloads: Vec<String>,
    #[serde(rename = "fileSize")]
    file_size: usize,
}

#[derive(Serialize)]
struct Dependencies {
    minecraft: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    forge: Option<String>,
    #[serde(rename = "fabric-loader", skip_serializing_if = "Option::is_none")]
    fabric: Option<String>,
    #[serde(rename = "quilt-loader", skip_serializing_if = "Option::is_none")]
    quilt: Option<String>,
}

#[derive(Serialize)]
struct Index {
    #[serde(rename = "formatVersion")]
    format_version: u8,
    game: String,
    #[serde(rename = "versionId")]
    version_id: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    files: Vec<ModFile>,
    dependencies: Dependencies,
}

#[tokio::main]
pub async fn execute() -> Result<(), Error> {
    let profile = Profile::get_selected()?;
    let file = File::create(format!(
        "{} v{}.mrpack",
        profile.config.name, profile.config.version
    ))?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default();

    // modrinth.index.json
    let index_content = serde_json::to_string(&Index {
        format_version: 1,
        game: "minecraft".into(),
        version_id: profile.config.version.clone(),
        name: profile.config.name.clone(),
        summary: None,
        files: profile
            .get_mods()?
            .par_iter()
            .map(|mcmod| ModFile {
                path: format!("mods/{}", mcmod.filename),
                hashes: FileHashes {
                    sha1: mcmod.download.sha1.clone(),
                    sha512: mcmod.download.sha512.clone(),
                },
                downloads: vec![mcmod.download.url.clone()],
                file_size: mcmod.download.file_size,
                env: Some(ModEnvironment {
                    client: mcmod.client_side.to_string(),
                    server: mcmod.server_side.to_string(),
                }),
            })
            .collect(),
        dependencies: Dependencies {
            minecraft: profile.config.versions.minecraft.clone(),
            forge: profile.config.versions.forge.clone(),
            fabric: profile.config.versions.fabric.clone(),
            quilt: profile.config.versions.quilt.clone(),
        },
    })?;

    zip.start_file("modrinth.index.json", options)?;
    zip.write_all(index_content.as_bytes())?;

    // overrides folder
    zip.add_directory("overrides", options)?;
    zip.add_directory("server-overrides", options)?;
    zip.add_directory("client-overrides", options)?;

    zip.finish()?;

    success(&format!(
        "Successfully exported {} {}!",
        profile.config.name, profile.config.version
    ));

    Ok(())
}
