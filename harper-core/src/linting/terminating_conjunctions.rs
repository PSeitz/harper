use super::{Lint, LintKind, PatternLinter};
use crate::patterns::{ConsumesRemainingPattern, Pattern, SequencePattern};
use crate::{Lrc, TokenStringExt};

pub struct TerminatingConjunctions {
    pattern: Box<dyn Pattern>
}

impl Default for TerminatingConjunctions {
    fn default() -> Self {
        Self {
            pattern: Box::new(ConsumesRemainingPattern::new(Box::new(
                SequencePattern::default()
                    .then_any_word_in(Lrc::new(
                        [
                            "after",
                            "although",
                            "as",
                            "because",
                            "before",
                            "if",
                            "lest",
                            "once",
                            "only",
                            "since",
                            "so",
                            "supposing",
                            "than",
                            "though",
                            "till",
                            "unless",
                            "until",
                            "when",
                            "whenever",
                            "where",
                            "whereas",
                            "wherever",
                            "whether",
                            "not",
                            "while",
                            "or",
                            "nor",
                            "so",
                            "and"
                        ]
                        .into_iter()
                        .collect()
                    ))
                    .then_comma()
            )))
        }
    }
}

impl PatternLinter for TerminatingConjunctions {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[crate::Token], source: &[char]) -> Lint {
        let span = matched_tokens.span().unwrap();
        let word = span.get_content_string(source);

        Lint {
            span,
            lint_kind: LintKind::Miscellaneous,
            suggestions: vec![],
            message: format!(
                "Subordinating conjunctions like “{word}” should not appear at the end of a \
                 clause."
            ),
            priority: 63
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TerminatingConjunctions;
    use crate::linting::tests::assert_lint_count;

    #[test]
    fn issue_131() {
        assert_lint_count(
            "More often than, we cannot foresee that of our community.",
            TerminatingConjunctions::default(),
            1
        )
    }

    #[test]
    fn no_false_positive() {
        assert_lint_count("Cookies and milk.", TerminatingConjunctions::default(), 0)
    }
}