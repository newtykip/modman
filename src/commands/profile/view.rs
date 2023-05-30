use chrono::NaiveDateTime;
use clap::Args as ClapArgs;
use modman::{
    utils::{bold, colour, underline, url},
    Error, Profile,
};
use owo_colors::colors::{Green, Red};

#[derive(ClapArgs)]
pub struct Args {
    id: Option<String>,
}

#[tokio::main]
pub async fn execute(args: Args) -> Result<(), Error> {
    let Profile { config, repo, .. } = match args.id {
        Some(id) => Profile::load(&id),
        None => Profile::load_selected(),
    }?;

    println!(
        "{}{}
Author: {}
Version: {}

Sync initialized: {}",
        underline(&bold(&config.name)),
        if let Some(summary) = config.summary {
            format!("\n{}\n", summary)
        } else {
            "".into()
        },
        config.author,
        config.version,
        bold(&if repo.is_some() {
            colour::<Green>("Yes")
        } else {
            colour::<Red>("No")
        })
    );

    // print repository info if it exists
    if let Some(repo) = repo {
        let destination_url = {
            let remote = repo.find_remote("origin");

            match remote {
                Ok(remote) => remote.url().map(|x| url(x)).unwrap_or_else(|| bold("N/A")),
                Err(_) => bold("N/A"),
            }
        };

        // find the most recent commit
        let mut revwalk = repo.revwalk()?;

        revwalk.set_sorting(git2::Sort::TIME)?;
        revwalk.push_head()?;

        let recent_commit = revwalk.nth(0).map(|oid| repo.find_commit(oid.unwrap()));

        let save_message: String = match &recent_commit {
            Some(Ok(commit)) => commit.message().unwrap().trim().into(),
            _ => "N/A".into(),
        };

        let saved_at = match &recent_commit {
            Some(Ok(commit)) => {
                let time = commit.time();
                let ms = 1000 * (time.seconds() + (time.offset_minutes() * 60) as i64);
                let datetime = NaiveDateTime::from_timestamp_millis(ms).unwrap();

                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            }
            _ => "N/A".into(),
        };

        println!(
            "Destination: {}
Saved At: {}
Message: {}",
            destination_url,
            saved_at,
            bold(&save_message),
        );
    }

    Ok(())
}
