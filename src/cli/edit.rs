//! Building and saving note text.

use std::io::Read;

use anyhow::{Result, bail};

use crate::note::link_key;
use crate::store::{SaveOutcome, Store};

pub(super) fn read_stdin() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

/// `#a #b` line for the note body: leading `#` and blanks stripped, empty tags dropped.
fn tag_line(tags: &[String]) -> Option<String> {
    let tags: Vec<String> = tags
        .iter()
        .map(|t| t.trim().trim_start_matches('#').trim())
        .filter(|t| !t.is_empty())
        .map(|t| format!("#{t}"))
        .collect();
    (!tags.is_empty()).then(|| tags.join(" "))
}

/// Text of a new note: `# title`, a tag line, then the body, blank-line separated.
pub(super) fn new_text(title: Option<&str>, tags: &[String], body: Option<&str>) -> String {
    let mut text = title.map(|t| format!("# {t}\n")).unwrap_or_default();
    if let Some(line) = tag_line(tags) {
        text.push_str(&format!("\n{line}\n"));
    }
    if let Some(body) = body {
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str(body);
    }
    text
}

/// `text` plus `body` as new final lines, exactly one newline between and after.
pub(super) fn appended(text: &str, body: &str) -> String {
    let mut t = text.to_string();
    if !t.is_empty() && !t.ends_with('\n') {
        t.push('\n');
    }
    t.push_str(body);
    if !body.ends_with('\n') {
        t.push('\n');
    }
    t
}

/// Save `text` into note `i` the way the TUI does: a conflict with a concurrent writer is an
/// error (the text survives as the conflict copy), a changed title rewrites `[[links]]` in
/// the other notes. Mirrors `App::save_tab`.
pub(super) fn save(store: &mut Store, i: usize, text: &str) -> Result<()> {
    let old_title = store.notes[i].title.clone();
    match store.save(i, text)? {
        SaveOutcome::Saved => {}
        SaveOutcome::Conflict(p) => bail!(
            "note changed on disk meanwhile; your text was saved as {}",
            p.display()
        ),
    }
    let new_title = store.notes[i].title.clone();
    if old_title != "Untitled" && link_key(&old_title) != link_key(&new_title) {
        let own = store.notes[i].path.clone();
        store.relink(&old_title, &new_title, &[own])?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_text_assembles_title_tags_and_body() {
        let tags = ["#work".to_string(), " ".into(), "home".into()];
        assert_eq!(new_text(Some("T"), &tags, None), "# T\n\n#work #home\n");
        assert_eq!(new_text(Some("T"), &[], Some("body\n")), "# T\n\nbody\n");
        assert_eq!(new_text(None, &[], Some("# Piped\n")), "# Piped\n");
        assert_eq!(new_text(None, &tags, Some("x")), "\n#work #home\n\nx");
    }

    #[test]
    fn appended_keeps_single_newlines() {
        assert_eq!(appended("# A\n\n- a", "- b"), "# A\n\n- a\n- b\n");
        assert_eq!(appended("# A\n", "- b\n"), "# A\n- b\n");
        assert_eq!(appended("", "x"), "x\n");
    }
}
