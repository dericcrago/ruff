use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_index::Indexer;
use ruff_source_file::Line;
use ruff_text_size::{TextLen, TextRange, TextSize};

use crate::Locator;
use crate::checkers::ast::LintContext;
use crate::registry::Rule;
use crate::settings::LinterSettings;
use crate::{AlwaysFixableViolation, Applicability, Edit, Fix};

/// ## What it does
/// Checks for superfluous trailing whitespace.
///
/// ## Why is this bad?
/// According to [PEP 8], "avoid trailing whitespace anywhere. Because it’s usually
/// invisible, it can be confusing"
///
/// ## Example
/// ```python
/// spam(1) \n#
/// ```
///
/// Use instead:
/// ```python
/// spam(1)\n#
/// ```
///
/// ## Fix Safety
///
/// This fix is marked unsafe if the whitespace is inside a multiline string,
/// as removing it changes the string's content.
///
/// [PEP 8]: https://peps.python.org/pep-0008/#other-recommendations
#[derive(ViolationMetadata)]
pub(crate) struct TrailingWhitespace;

impl AlwaysFixableViolation for TrailingWhitespace {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Trailing whitespace".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove trailing whitespace".to_string()
    }
}

/// ## What it does
/// Checks for superfluous whitespace in blank lines.
///
/// ## Why is this bad?
/// According to [PEP 8], "avoid trailing whitespace anywhere. Because it’s usually
/// invisible, it can be confusing"
///
/// ## Example
/// ```python
/// class Foo(object):\n    \n    bang = 12
/// ```
///
/// Use instead:
/// ```python
/// class Foo(object):\n\n    bang = 12
/// ```
///
/// ## Fix Safety
///
/// This fix is marked unsafe if the whitespace is inside a multiline string,
/// as removing it changes the string's content.
///
/// [PEP 8]: https://peps.python.org/pep-0008/#other-recommendations
#[derive(ViolationMetadata)]
pub(crate) struct BlankLineWithWhitespace;

impl AlwaysFixableViolation for BlankLineWithWhitespace {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Blank line contains whitespace".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove whitespace from blank line".to_string()
    }
}

/// W291, W293
pub(crate) fn trailing_whitespace(
    line: &Line,
    locator: &Locator,
    indexer: &Indexer,
    settings: &LinterSettings,
    context: &LintContext,
) {
    let whitespace_len: TextSize = line
        .chars()
        .rev()
        .take_while(|c| c.is_whitespace())
        .map(TextLen::text_len)
        .sum();
    if whitespace_len > TextSize::from(0) {
        let range = TextRange::new(line.end() - whitespace_len, line.end());
        // Removing trailing whitespace is not safe inside multiline strings.
        let applicability = if indexer.multiline_ranges().contains_range(range) {
            Applicability::Unsafe
        } else {
            Applicability::Safe
        };
        if range == line.range() {
            if settings.rules.enabled(Rule::BlankLineWithWhitespace) {
                let mut diagnostic = context.report_diagnostic(BlankLineWithWhitespace, range);
                // Remove any preceding continuations, to avoid introducing a potential
                // syntax error.
                diagnostic.set_fix(Fix::applicable_edit(
                    Edit::range_deletion(TextRange::new(
                        indexer
                            .preceded_by_continuations(line.start(), locator.contents())
                            .unwrap_or(range.start()),
                        range.end(),
                    )),
                    applicability,
                ));
            }
        } else if settings.rules.enabled(Rule::TrailingWhitespace) {
            let mut diagnostic = context.report_diagnostic(TrailingWhitespace, range);
            diagnostic.set_fix(Fix::applicable_edit(
                Edit::range_deletion(range),
                applicability,
            ));
        }
    }
}
