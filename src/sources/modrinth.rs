use crate::{
    enums::{DependencyType, Loader, Sources},
    Mod,
};
use async_trait::async_trait;
use ferinth::{structures::version::DependencyType as FerinthDependency, Ferinth};

#[async_trait]
pub trait FromModrinth {
    /// Get a mod from Modrinth by ID.
    async fn from_modrinth(
        client: &Ferinth,
        id: &str,
        loader: Loader,
        game_versions: Option<&[&str]>,
        featured: Option<bool>,
    ) -> Result<Mod, Box<dyn std::error::Error>>;
}

#[async_trait]
impl FromModrinth for Mod {
    async fn from_modrinth(
        client: &Ferinth,
        id: &str,
        loader: Loader,
        game_versions: Option<&[&str]>,
        featured: Option<bool>,
    ) -> Result<Mod, Box<dyn std::error::Error>> {
        let project = client.get_project(id).await?;
        let versions = client
            .list_versions_filtered(id, Some(&[loader.as_str()]), game_versions, featured)
            .await?;
        let latest = &versions[0];
        let download = &latest.files[0];

        Ok(Mod {
            name: project.title,
            filename: download.filename.clone(),
            url: download.url.to_string(),
            dependencies: latest
                .dependencies
                .iter()
                .map(|dependency| {
                    (
                        match dependency.dependency_type {
                            FerinthDependency::Embedded => DependencyType::Embedded,
                            FerinthDependency::Incompatible => DependencyType::Incompatible,
                            FerinthDependency::Optional => DependencyType::Optional,
                            _ => DependencyType::Required,
                        },
                        dependency.version_id.clone().unwrap(),
                    )
                })
                .collect::<Vec<(DependencyType, String)>>(),
            source: Sources::Modrinth,
        })
    }
}
