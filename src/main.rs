use modman::{Download, FromCurse, Loader, Mod};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // let ferinth = ferinth::Ferinth::default();
    // let zoomify = Mod::from_modrinth(&ferinth, "zoomify", Loader::Quilt, None, None)
    //     .await
    //     .unwrap();

    let client = reqwest::Client::default();
    let screenshot = Mod::from_curseforge(&client, "botania-fabric", Loader::Quilt, None)
        .await
        .unwrap();

    screenshot
        .download(&client, PathBuf::from("example/mods"))
        .await
        .unwrap();

    // println!("{:?}", screenshot);
}
