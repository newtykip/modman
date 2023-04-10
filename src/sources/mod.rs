use self::{curseforge::CurseMod, modrinth::ModrinthMod};
use crate::{Error, GameVersions, Loader, Mod};

pub mod curseforge;
pub mod modrinth;

#[derive(Debug)]
pub struct SearchResult {
    pub name: String,
    pub id: String,
}

impl SearchResult {
    async fn resolve_curseforge(
        &self,
        loader: Loader,
        game_versions: GameVersions,
    ) -> Result<CurseMod, Error> {
        Ok(
            Mod::from_curseforge(self.id.as_str(), loader, game_versions, None, None)
                .await?
                .unwrap(),
        )
    }

    async fn resolve_modrinth(
        &self,
        loader: Loader,
        game_versions: GameVersions,
    ) -> Result<ModrinthMod, Error> {
        Ok(
            Mod::from_modrinth(self.id.as_str(), loader, game_versions, None, None)
                .await?
                .unwrap(),
        )
    }
}
