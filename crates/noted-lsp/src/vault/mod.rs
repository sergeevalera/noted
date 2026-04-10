pub mod index;
pub mod parser;
pub mod scanner;

pub use index::{
    build_index, resolve_wikilink, Frontmatter, Heading, LinkReference, NoteEntry, Tag, VaultIndex,
};
pub use parser::parse_note;
pub use scanner::scan_vault;
