pub mod curseforge;
pub mod modrinth;

#[derive(Debug)]
pub struct SearchResult {
    name: String,
    id: String,
}
