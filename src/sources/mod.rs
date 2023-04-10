pub mod curseforge;
pub mod modrinth;

#[derive(Debug, Clone)]
pub struct SearchResult {
    name: String,
    id: String,
}
