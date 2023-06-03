use chrono::NaiveDateTime;
use clap::Args as ClapArgs;
use modman::{
    utils::{bold, colour, create_slug, underline, url},
    Error, Profile,
};
use owo_colors::colors::{Green, Red};

#[derive(ClapArgs)]
pub struct Args {
    name: Option<String>,
}

#[tokio::main]
pub async fn execute(args: Args) -> Result<(), Error> {
    let Profile { config, repo, .. } = match args.name {
        Some(name) => Profile::load(&create_slug(&name)),
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
                Ok(remote) => remote.url().map(url).unwrap_or_else(|| "N/A".into()),
                Err(_) => "N/A".into(),
            }
        };

        // find the most recent commit
        let mut revwalk = repo.revwalk()?;

        revwalk.set_sorting(git2::Sort::TIME)?;

        let push_result = revwalk.push_head();

        let saved_at: String;
        let save_message: String;

        if push_result.is_ok() {
            push_result.unwrap();

            let recent_commit = revwalk.next().map(|oid| repo.find_commit(oid.unwrap()));

            save_message = match &recent_commit {
                Some(Ok(commit)) => commit.message().unwrap().trim().into(),
                _ => "N/A".into(),
            };

            saved_at = match &recent_commit {
                Some(Ok(commit)) => {
                    let time = commit.time();
                    let ms = 1000 * (time.seconds() + (time.offset_minutes() * 60) as i64);
                    let datetime = NaiveDateTime::from_timestamp_millis(ms).unwrap();

                    bold(&datetime.format("%Y-%m-%d %H:%M:%S").to_string())
                }
                _ => "N/A".into(),
            };
        } else {
            save_message = "N/A".into();
            saved_at = "N/A".into();
        }

        println!(
            "Destination: {}
Saved At: {}
Message: {}",
            destination_url, saved_at, save_message,
        );
    };

    Ok(())
}
