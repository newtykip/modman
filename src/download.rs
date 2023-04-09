use crate::Mod;
use async_trait::async_trait;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::{fs::File, io::Write, path::PathBuf};

#[async_trait]
pub trait Download {
    async fn download(
        &self,
        client: &Client,
        directory: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl Download for Mod {
    async fn download(
        &self,
        client: &Client,
        path: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if path.is_file() {
            panic!("path must point towards a folder");
        }

        // fetch the mod
        let res = client.get(&self.url).send().await?;
        let total_size = res.content_length().unwrap();

        // set up progress bar
        let progress = ProgressBar::new(total_size);

        progress.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        .progress_chars("#>-"));
        progress.set_message(format!(
            "Downloading {} from {}",
            self.name,
            self.source.to_string()
        ));

        // download chunks
        std::fs::create_dir_all(&path)?;

        let path = &path.join(&self.filename);

        let mut file = File::create(path)?;
        let mut downloaded = 0u64;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.unwrap();
            file.write_all(&chunk)?;

            let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            progress.set_position(new);
        }

        progress.finish_with_message(format!(
            "Downloaded {} from {} to {}",
            self.name,
            self.source.to_string(),
            path.to_str().unwrap()
        ));

        Ok(())
    }
}
