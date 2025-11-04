use clap::Parser;

/// This only handles the *field*'s formatting, not the contents of
/// it. Those tasks will be handled later.
pub struct FieldFormatArgs {
    /// The width of the field for field names, in min/max order.
    pub width: Option<u16>,
    /// Sort the fields alphabetically.
    pub alphabetical: bool,
    /// In what order to put the fields in back into the bibliography.
    pub order: Vec<String>,
    /// What fields to remove, if any. Will be removed *after* being sorted.
    pub remove: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct Args {
    /// Generate bibtex instead of biblatex
    #[arg(short, long)]
    pub bibtex: bool,
    /// Indent the entries' keys in the final output file.
    #[arg(long)]
    pub indent: bool,
    /// Align the values on '=' for the entries.
    #[arg(long)]
    pub align: bool,
    /// Break-up field lists so they end up as 'one field per line'.
    #[arg(long)]
    pub break_fields: bool,
    /// Output filename (leave empty to print to stdout)
    #[arg(short, long)]
    pub output_filename: Option<String>,
    /// Edit input filename in-place
    #[arg(short, long)]
    pub inplace: bool,
    /// Input filename (leave empty to read from stdin)
    pub filename: Option<String>,
}

impl FieldFormatArgs {

    /// Default values for the formatting args. Does not touch anything!
    pub fn init() -> FieldFormatArgs {
        FieldFormatArgs{
            width: None,
            alphabetical: false,
            order: vec![],
            remove: vec![],
        }
    }

    /// Loads the field formatting arguments from the given file.
    pub fn from_file() -> Result<FieldFormatArgs, Option<String>> {
        Ok(FieldFormatArgs::init())
    }

}
