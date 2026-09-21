use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

struct Vault {
    home: PathBuf,
    dir: PathBuf,
}

impl Vault {
    fn new() -> Self {
        let home = std::env::temp_dir().join(format!(
            "tnotes-cli-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let dir = home.join("vault");
        fs::create_dir_all(&dir).unwrap();
        Vault { home, dir }
    }

    fn write(&self, rel: &str, text: &str) {
        let p = self.dir.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, text).unwrap();
    }

    fn cmd(&self, args: &[&str]) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_tnotes"));
        c.env("XDG_CONFIG_HOME", self.home.join(".config"))
            .env("XDG_STATE_HOME", self.home.join(".state"))
            .arg("--dir")
            .arg(&self.dir)
            .args(args);
        c
    }

    fn run(&self, args: &[&str]) -> Output {
        self.cmd(args).output().unwrap()
    }

    fn ok(&self, args: &[&str]) -> String {
        let out = self.run(args);
        assert!(
            out.status.success(),
            "{args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    }

    /// Run with `input` on stdin; asserts success and returns stdout.
    fn ok_stdin(&self, args: &[&str], input: &str) -> String {
        let mut child = self
            .cmd(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(
            out.status.success(),
            "{args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    }
}

impl Drop for Vault {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.home);
    }
}

fn path_str(p: &Path) -> &str {
    p.to_str().unwrap()
}

#[test]
fn new_ls_cat_trash_roundtrip() {
    let v = Vault::new();
    assert_eq!(
        v.ok(&["new", "Weekly plan", "--tag", "work"]),
        "vault/weekly-plan\n"
    );
    assert_eq!(v.ok(&["ls"]), "vault/weekly-plan\tWeekly plan\t#work\n");
    assert_eq!(v.ok(&["cat", "weekly-plan"]), "# Weekly plan\n\n#work\n");
    assert_eq!(
        v.ok(&["trash", "weekly-plan"]),
        "trashed vault/weekly-plan\n"
    );
    assert!(v.dir.join(".Trash").join("weekly-plan.md").exists());
    assert_eq!(v.ok(&["ls"]), "");
}

#[test]
fn json_lists_links_and_backlinks() {
    let v = Vault::new();
    v.write("a.md", "# A\n\n[[B]]\n");
    v.write("b.md", "# B\n");
    let out: serde_json::Value = serde_json::from_str(&v.ok(&["ls", "--json"])).unwrap();
    let by_title = |t: &str| {
        out.as_array()
            .unwrap()
            .iter()
            .find(|n| n["title"] == t)
            .unwrap()
            .clone()
    };
    assert_eq!(by_title("B")["backlinks"], serde_json::json!(["vault/a"]));
    assert_eq!(by_title("A")["links"], serde_json::json!(["B"]));
    assert_eq!(by_title("A")["folder"], "vault");
    assert!(by_title("A").get("text").is_none());
    let one: serde_json::Value = serde_json::from_str(&v.ok(&["cat", "a", "--json"])).unwrap();
    assert_eq!(one["text"], "# A\n\n[[B]]\n");
    // The global flag works before the subcommand too.
    let out2 = v.ok(&["--json", "ls"]);
    assert!(out2.trim_start().starts_with('['));
}

#[test]
fn new_normalises_tags_and_rejects_blank_titles() {
    let v = Vault::new();
    v.ok(&[
        "new", "T", "--tag", "#work", "--tag", " ", "--tag", " home ",
    ]);
    assert_eq!(v.ok(&["cat", "t"]), "# T\n\n#work #home\n");
    for title in ["", "   "] {
        let out = v.run(&["new", title]);
        assert_eq!(out.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&out.stderr).contains("give a title"));
    }
    assert_eq!(v.ok(&["ls"]).lines().count(), 1);
}

#[test]
fn ambiguous_stem_is_an_error() {
    let v = Vault::new();
    v.write("x.md", "# Root x\n");
    v.write("sub/x.md", "# Sub x\n");
    let out = v.run(&["cat", "x"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("ambiguous"));
    assert_eq!(v.ok(&["cat", "vault/sub/x"]), "# Sub x\n");
    assert_eq!(v.ok(&["cat", path_str(&v.dir.join("x.md"))]), "# Root x\n");
    let out = v.run(&["cat", "nope"]);
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn search_ranks_matches() {
    let v = Vault::new();
    v.write("groceries.md", "# Groceries\n\nmilk\n");
    v.write("plan.md", "# Plan\n");
    assert_eq!(v.ok(&["search", "milk"]), "vault/groceries\tGroceries\t\n");
    assert_eq!(v.ok(&["ls", "--folder", "vault"]).lines().count(), 2);
}

#[test]
fn new_from_stdin_without_title() {
    let v = Vault::new();
    let mut child = v
        .cmd(&["new", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"# Piped\n\nbody\n")
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    assert_eq!(
        fs::read_to_string(v.dir.join("piped.md")).unwrap(),
        "# Piped\n\nbody\n"
    );
    let out = v.run(&["new"]);
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn write_replaces_text_and_renames_on_title_change() {
    let v = Vault::new();
    v.write("b.md", "# B\n");
    v.write("a.md", "# A\n\nsee [[B]]\n");
    assert_eq!(
        v.ok_stdin(&["write", "b", "--stdin"], "# Bee\n\nbody\n"),
        "wrote vault/bee\n"
    );
    assert!(v.dir.join("bee.md").exists());
    assert!(!v.dir.join("b.md").exists());
    assert_eq!(
        fs::read_to_string(v.dir.join("a.md")).unwrap(),
        "# A\n\nsee [[Bee]]\n"
    );
    let one: serde_json::Value =
        serde_json::from_str(&v.ok_stdin(&["write", "bee", "--stdin", "--json"], "# Bee\n\nv2\n"))
            .unwrap();
    assert_eq!(one["text"], "# Bee\n\nv2\n");
    assert_eq!(one["backlinks"], serde_json::json!(["vault/a"]));
    // --stdin is mandatory for write.
    assert_eq!(v.run(&["write", "bee"]).status.code(), Some(2));
}

#[test]
fn append_adds_lines_with_single_newlines() {
    let v = Vault::new();
    v.write("plan.md", "# Plan\n\n- [ ] a");
    assert_eq!(
        v.ok(&["append", "plan", "- [ ] b"]),
        "appended to vault/plan\n"
    );
    v.ok_stdin(&["append", "plan", "--stdin"], "- [ ] c\n");
    assert_eq!(
        fs::read_to_string(v.dir.join("plan.md")).unwrap(),
        "# Plan\n\n- [ ] a\n- [ ] b\n- [ ] c\n"
    );
    let out = v.run(&["append", "plan"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("give text or --stdin"));
}

#[test]
fn restore_brings_a_note_back() {
    let v = Vault::new();
    v.write("sub/x.md", "# X\n");
    v.ok(&["trash", "x"]);
    assert!(v.dir.join(".Trash/sub/x.md").exists());
    assert_eq!(v.ok(&["restore", "vault/sub/x"]), "restored vault/sub/x\n");
    assert!(v.dir.join("sub/x.md").exists());
    assert_eq!(v.ok(&["ls"]).lines().count(), 1);
    v.ok(&["trash", "x"]);
    assert_eq!(
        v.ok(&["restore", "vault/.Trash/sub/x"]),
        "restored vault/sub/x\n"
    );
    assert_eq!(v.run(&["restore", "x"]).status.code(), Some(1));
}

#[test]
fn limit_caps_the_list() {
    let v = Vault::new();
    for n in ["a", "b", "c"] {
        v.write(&format!("{n}.md"), &format!("# {n}\n"));
    }
    assert_eq!(v.ok(&["ls", "--limit", "2"]).lines().count(), 2);
    assert_eq!(v.ok(&["search", "a", "--limit", "1"]).lines().count(), 1);
    assert_eq!(v.ok(&["ls", "--limit", "0"]), "");
}

#[test]
fn new_refuses_duplicate_titles_unless_forced() {
    let v = Vault::new();
    v.ok(&["new", "Plan"]);
    let out = v.run(&["new", "plan"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("already exists: vault/plan"));
    assert_eq!(v.ok(&["new", "Plan", "--duplicate"]), "vault/plan-2\n");
}
