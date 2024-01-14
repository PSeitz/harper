mod lint;
mod sentence_capitalization;
mod spell_check;
mod unclosed_quotes;

pub use lint::{Lint, LintKind, Suggestion};

use crate::Document;

use self::lint::Linter;

pub fn all_linters(document: &Document) -> Vec<Lint> {
    let mut lints = Vec::new();

    let linters: [Linter; 3] = [
        spell_check::spell_check,
        sentence_capitalization::sentence_capitalization_lint,
        unclosed_quotes::unclosed_quotes,
    ];

    for linter in linters {
        lints.append(&mut linter(document));
    }

    lints
}
