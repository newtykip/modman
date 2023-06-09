use clap::Args as ClapArgs;
use modman::{Config, Error, CONFIG_CENSOR, CONFIG_TYPES};
use prettytable::Table;

const CENSOR_STRING: &str = "********";

#[derive(ClapArgs)]
pub struct Args {
    /// Should sensitive information be uncensored?
    #[arg(short)]
    uncensor: bool,
}

// todo: make this look good

#[tokio::main]
pub async fn execute(args: Args) -> Result<(), Error> {
    let config = Config::load()?;
    let mut table = Table::new();

    table.add_row(row!["Key", "Value"]);

    for key in CONFIG_TYPES.keys() {
        let value = if let Some(value) = config.0.get(*key) {
            println!("{:?}", value);

            if let Some(value) = value {
                if args.uncensor && *CONFIG_CENSOR.get(*key).unwrap() {
                    value.clone()
                } else {
                    CENSOR_STRING.into()
                }
            } else {
                "N/A".into()
            }
        } else {
            "N/A".into()
        };

        table.add_row(row![key, value]);
    }

    table.printstd();

    Ok(())
}
