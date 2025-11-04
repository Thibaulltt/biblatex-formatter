use biblatex::{Entry};

/// This only handles the *field*'s formatting, not the contents of
/// it. Those tasks will be handled later.
#[allow(dead_code)]
pub struct FormatArgs {
    /// The width of the field for field names, in min/max order.
    pub width: Option<u16>,
    /// In what order to put the fields in back into the bibliography.
    pub order: Vec<String>,
    /// What fields to remove, if any. Will be removed *after* being sorted.
    pub remove: Vec<String>,
}

#[allow(dead_code)]
impl FormatArgs {

    /// Default values for the formatting args. Does not touch anything!
    pub fn init() -> FormatArgs {
        FormatArgs{
            width: None,
            order: vec![],
            remove: vec![],
        }
    }

    pub fn resolve(&mut self) -> &FormatArgs {
        self.width = self.width.or(Some(30));
        if self.order.is_empty() { self.order = vec!["*".to_string()]; }
        self
    }

    /// Loads the field formatting arguments from the given file.
    pub fn from_file() -> Result<FormatArgs, Option<String>> {
        Ok(FormatArgs::init())
    }

}

/// Resolve a given `entry` with the provided `config` for each field.
#[allow(unused_variables,dead_code)]
pub fn resolve_entry(entry: &Entry, config: FormatArgs) -> Entry {
    todo!() // TODO
}

#[allow(unused_variables,dead_code)]
pub fn to_escaped_string(contents: &String) -> String {
    todo!() // TODO
}
