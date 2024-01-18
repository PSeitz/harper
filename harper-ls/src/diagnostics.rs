use harper_core::{all_linters, Dictionary, Document, Lint, Span, Suggestion};
use lsp_types::{CodeAction, CodeActionKind, Diagnostic, Location, Position, Range, Url};
use std::{fs::read, io::Read};

pub fn generate_diagnostics(file_url: &Url) -> anyhow::Result<Vec<Diagnostic>> {
    let file_str = open_url(file_url)?;
    let source_chars: Vec<_> = file_str.chars().collect();
    let lints = lint_string(&file_str);

    let diagnostics = lints
        .into_iter()
        .map(|lint| lint_to_diagnostic(lint, &source_chars))
        .collect();

    Ok(diagnostics)
}

pub fn generate_code_actions(url: &Url, range: Range) -> anyhow::Result<Vec<CodeAction>> {
    let file_str = open_url(url)?;
    let source_chars: Vec<_> = file_str.chars().collect();
    let lints = lint_string(&file_str);

    // Find lints whose span overlaps with range
    let span = range_to_span(&source_chars, range);

    let actions = lints
        .into_iter()
        .filter(|lint| lint.span.overlaps_with(span))
        .flat_map(|lint| lint_to_code_actions(&lint).collect::<Vec<_>>())
        .collect();

    Ok(actions)
}

fn lint_to_code_actions(lint: &Lint) -> impl Iterator<Item = CodeAction> + '_ {
    lint.suggestions.iter().map(|suggestion| CodeAction {
        title: suggestion.to_string(),
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: None,
        edit: None,
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    })
}

fn open_url(url: &Url) -> anyhow::Result<String> {
    let file = read(url.path())?;
    Ok(String::from_utf8(file)?)
}

fn lint_string(text: &str) -> Vec<Lint> {
    let document = Document::new(text);
    let dictionary = Dictionary::new();
    all_linters(&document, dictionary)
}

fn lint_to_diagnostic(lint: Lint, source: &[char]) -> Diagnostic {
    let range = span_to_range(source, lint.span);

    Diagnostic {
        range,
        severity: None,
        code: None,
        code_description: None,
        source: Some("Harper".to_string()),
        message: lint.message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn span_to_range(source: &[char], span: Span) -> Range {
    let start = index_to_position(source, span.start);
    let end = index_to_position(source, span.end);

    Range { start, end }
}

fn index_to_position(source: &[char], index: usize) -> Position {
    let before = &source[0..index];
    let newline_indices: Vec<_> = before
        .iter()
        .enumerate()
        .filter_map(|(idx, c)| if *c == '\n' { Some(idx) } else { None })
        .collect();

    let lines = newline_indices.len();
    let cols = index - newline_indices.last().copied().unwrap_or(1) - 1;

    Position {
        line: lines as u32,
        character: cols as u32,
    }
}

fn position_to_index(source: &[char], position: Position) -> usize {
    let newline_indices =
        source
            .iter()
            .enumerate()
            .filter_map(|(idx, c)| if *c == '\n' { Some(idx) } else { None });

    let line_start_idx = newline_indices
        .take(position.line as usize)
        .next()
        .unwrap_or(0);
    line_start_idx + position.character as usize
}

fn range_to_span(source: &[char], range: Range) -> Span {
    let start = position_to_index(source, range.start);
    let end = position_to_index(source, range.end);

    Span::new(start, end)
}