use clap::Parser;
use log::{debug, error, info};

#[derive(Parser, Debug)]
struct Args {
    #[arg(name = "files", required = true)]
    files: Vec<String>,
}

fn tokenize_file(filename: &str) -> anyhow::Result<Option<rusjure_tokens::TokenStream>> {
    debug!("Reading file: `{}`", filename);

    let src = match std::fs::read_to_string(filename) {
        Ok(ok) => ok,
        Err(err) => {
            error!(
                "A filesystem error occurred during reading file `{}`: {}",
                filename, err
            );
            return Ok(None);
        }
    };

    debug!("Successfully read file `{}`", filename);

    let tt = rusjure_lexer::parse(&src)?;

    debug!("Successfully tokenized file `{}`", filename);

    Ok(Some(tt))
}

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .try_init()?;

    let args: Args = Args::parse();
    let parser = rusjure_parser::Parser::default();

    info!("Starting up...");

    for filename in args.files.iter() {
        if let Some(ts) = tokenize_file(filename)? {
            for tt in ts.iter() {
                parser.run(tt);
            }
        }
    }

    Ok(())
}
