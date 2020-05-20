extern crate cc;

use std::env;
use std::path::Path;
use std::process::Command;

fn is_git_worktree() -> bool {
    let cwd = &env::current_dir().unwrap();
    if Path::new(&cwd.join(".git")).exists() {
        return true;
    }
    if let Some(v) = cwd.parent() {
        return Path::new(&v.join(".git")).exists();
    }
    false
}

fn main() {
    if is_git_worktree() && !Path::new("nuklear-c/nuklear/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status()
            .unwrap();
    }
    cc::Build::new()
        .file("nuklear-c/bind.c")
        //.flag("-Wno-unused-parameter")
        //.flag("-Wno-implicit-fallthrough")
        //.flag("-Wno-unused-but-set-variable")
        .compile("libnuklear");
}
