use clap::Args as ClapArgs;
use modman::{
    utils::{bold, error, success},
    Config, Error, ValueType, CONFIG_TYPES,
};

#[derive(ClapArgs)]
pub struct Args {
    key: String,
    value: String,
}

// todo: fuzzy search keys

#[tokio::main]
pub async fn execute(args: Args) -> Result<(), Error> {
    let mut config = Config::load()?;

    // validate that the key exists
    if !CONFIG_TYPES
        .keys()
        .collect::<Vec<_>>()
        .contains(&&args.key.as_str())
    {
        error("Key not found");
        return Ok(());
    }

    // validate that the value is of the correct type
    let value_type = CONFIG_TYPES.get(&args.key).unwrap();

    match value_type {
        ValueType::String => {}
    }

    // set the config values
    config
        .0
        .entry(args.key.clone())
        .or_insert(Some(args.value.clone()));

    config.save()?;

    success(&format!(
        "Successfully updated setting {} to {}",
        bold(&args.key),
        bold(&args.value)
    ));

    Ok(())
}
