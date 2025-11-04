use clap::Parser;

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
