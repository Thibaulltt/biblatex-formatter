use std::io::Read;

use anyhow::{anyhow, Context, Result};
use clap::Parser;

use biblatex::{Bibliography, Chunk};

mod args;
mod fields;

fn main() -> Result<()> {
    // Rough outline of the order of things:
    // 1. Open the config file, and parse arguments (as it overrides config).
    // 2. Open the input file, or stdin.
    // 3. Parse the bibliography with the biblatex crate (black box).
    // 4. Actually format the entries with the given configuration.
    //   - Note: need to ignore comments, and @PREAMBLE @COMMENT keys
    //   - Note: need to ignore unknown/unspecified fields.
    // 5. Write the resulting bibliography entries to a file.

    let args = args::Args::parse();

    let content = match args.filename {
        Some(ref filename) => std::fs::read_to_string(filename)?,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let bibliography = Bibliography::parse(&content)
        .map_err(|err| anyhow!("error parsing bibliography: {}", err))?;

    let mut output_file: Box<dyn std::io::Write> = if args.inplace {
        let filename = &args
            .filename
            .context("cannot use --inplace when reading from stdin")?;
        let file = std::fs::File::create(filename)?;
        Box::from(file)
    } else if args.output_filename.is_some() {
        let file = std::fs::File::create(args.output_filename.unwrap())?;
        Box::from(file)
    } else {
        Box::from(std::io::stdout())
    };

    let mut format_config = fields::FormatArgs::init();
    format_config.resolve();

    for entry in bibliography.iter() {
        let needle_key = "ka:".to_string();
        if entry.key.eq(&needle_key) {
            println!("Key {needle_key} found. Fields: {{");
            // Read-only:
            for (field, chunks) in &entry.fields {
                print!("\t{field: <width$}= {{", field = field, width = format_config.width.unwrap().into());
                // Read-only:
                for chunk in chunks {
                    match &chunk.v {
                        Chunk::Normal(cn) => print!("Normal({cn}), "),
                        Chunk::Verbatim(cv) => print!("Verbatim({cv}), "),
                        Chunk::Math(cm) => print!("Math({cm}), "),
                    };
                }
                println!("}}");
            }
            println!("}}");
        }
    }

    let formatted = match args.bibtex {
        true => bibliography.to_bibtex_string(),
        false => bibliography.to_biblatex_string(),
    };

    output_file.write_all(formatted.as_bytes())?;

    Ok(())
}
