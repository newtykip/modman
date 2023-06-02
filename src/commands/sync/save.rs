use git2::{Config, IndexAddOption, PushOptions, RemoteCallbacks, Status};
use git2_credentials::CredentialHandler;
use inquire::validator::Validation;
use modman::{
    utils::{info, success},
    Error, Mod, Profile,
};
use rayon::prelude::*;
use std::path::PathBuf;

// todo: add default commit message

#[tokio::main]
pub async fn execute(profile: Profile) -> Result<(), Error> {
    // we know that the profile has a repo because we checked in the parent function
    let repo = profile.repo.unwrap();
    let repo_path = repo.path().parent().unwrap();
    let recent_commit = repo.head()?.peel_to_commit()?;
    let tree = recent_commit.tree()?;

    // find all of the changed files
    let statuses = repo.statuses(None)?;
    let changed_files = statuses
        .iter()
        .map(|x| (PathBuf::from(x.path().unwrap()), x.status()))
        .collect::<Vec<_>>();

    if changed_files.is_empty() {
        info("No changes to save (:");
        return Ok(());
    }

    let mut summary = String::new();

    // print out the added mods
    let mut added_mods = changed_files
        .iter()
        .filter(|x| x.1 == Status::INDEX_NEW || x.1 == Status::WT_NEW)
        .map(|x| x.0.clone())
        .map(|path| repo_path.join(path))
        .filter(|path| path.file_name().is_some())
        .map(|path| {
            let contents: Mod = toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
            contents.name
        })
        .peekable();

    if added_mods.peek().is_some() {
        summary += &format!(
            "Added the following mods:\n{}\n",
            added_mods
                .map(|x| format!("  ● {}\n", x))
                .collect::<String>()
        );
    }

    // print out the updated mods
    let mut updated_mods = changed_files
        .iter()
        .filter(|x| x.1 == Status::INDEX_MODIFIED || x.1 == Status::WT_MODIFIED)
        .map(|x| x.0.clone())
        .map(|path| repo_path.join(path))
        .filter(|path| path.file_name().is_some())
        .map(|path| {
            let contents: Mod = toml::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
            let previous_contents: Mod = toml::from_str(
                &String::from_utf8(
                    tree.get_path(&PathBuf::new().join("mods").join(path.file_name().unwrap()))
                        .unwrap()
                        .to_object(&repo)
                        .unwrap()
                        .as_blob()
                        .unwrap()
                        .content()
                        .to_vec(),
                )
                .unwrap(),
            )
            .unwrap();

            (contents.name, previous_contents.version, contents.version)
        })
        .peekable();

    if updated_mods.peek().is_some() {
        summary += &format!(
            "Updated the following mods:\n{}\n",
            updated_mods
                .map(|(name, previous, new)| format!("  ● {} from {} to {}", name, previous, new))
                .collect::<String>()
        );
    }

    // print out the removed mods
    let mut removed_mods = changed_files
        .iter()
        .filter(|x| x.1 == Status::INDEX_DELETED || x.1 == Status::WT_DELETED)
        .map(|x| x.0.clone())
        .map(|path| repo_path.join(path))
        .filter(|path| path.file_name().is_some())
        .map(|path| {
            let contents: Mod = toml::from_str(
                &String::from_utf8(
                    tree.get_path(&PathBuf::new().join("mods").join(path.file_name().unwrap()))
                        .unwrap()
                        .to_object(&repo)
                        .unwrap()
                        .as_blob()
                        .unwrap()
                        .content()
                        .to_vec(),
                )
                .unwrap(),
            )
            .unwrap();

            contents.name
        })
        .peekable();

    if removed_mods.peek().is_some() {
        summary += &format!(
            "Removed the following mods:\n{}",
            removed_mods
                .map(|x| format!("  ● {}\n", x))
                .collect::<String>()
        );
    }

    let mut summary = summary.chars();
    summary.next_back();
    summary.next_back();
    let summary = summary.collect::<String>();

    println!("{}", &summary);

    // get a commit message
    let commit_message = inquire::Text::new("Please enter a save message:")
        .with_validator(|message: &str| {
            if message.is_empty() {
                Ok(Validation::Invalid("Please enter a message".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()?;

    // git add .
    let mut index = repo.index()?;
    index.add_all(
        changed_files
            .par_iter()
            .map(|x| x.0.clone().into_os_string().into_string().unwrap())
            .collect::<Vec<_>>(),
        IndexAddOption::DEFAULT,
        None,
    )?;
    index.write()?;

    // git commit -m "message"
    let signature = repo.signature()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let head = repo.head()?;
    let parent = head.peel_to_commit()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &commit_message,
        &tree,
        &[&parent],
    )?;

    // git push origin main
    let mut success_message = "Successfully saved profile changes".to_string();

    if let Ok(mut remote) = repo.find_remote("origin") {
        let git_config = Config::open_default()?;
        let mut ch = CredentialHandler::new(git_config);
        let mut callbacks = RemoteCallbacks::new();

        callbacks.credentials(move |url, username, allowed| {
            ch.try_next_credential(url, username, allowed)
        });

        remote.push(
            &["refs/heads/main:refs/heads/main"],
            Some(PushOptions::new().remote_callbacks(callbacks)),
        )?;

        success_message += "and pushed to remote";
    }

    success(success_message.as_str());

    Ok(())
}
