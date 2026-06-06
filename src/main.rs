use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::PathBuf;
use std::process::{Command, exit};

const REPO: &str = "https://github.com/drmarceloclipi-star/define-problem";
const PLUGIN_NAME: &str = "define-problem";

fn main() {
    let home = std::env::var("HOME").unwrap_or_else(|_| {
        eprintln!("HOME not set");
        exit(1);
    });

    let src_dir: PathBuf = [&home, ".claude", "plugins-src", PLUGIN_NAME].iter().collect();
    let skills_dir: PathBuf = [&home, ".claude", "skills"].iter().collect();
    let link_path = skills_dir.join(PLUGIN_NAME);

    fs::create_dir_all(&skills_dir).expect("failed to create skills dir");
    fs::create_dir_all(src_dir.parent().unwrap()).expect("failed to create plugins-src dir");

    if src_dir.join(".git").exists() {
        println!("Updating...");
        run("git", &["-C", src_dir.to_str().unwrap(), "pull", "--ff-only"]);
    } else {
        println!("Cloning...");
        run("git", &["clone", REPO, src_dir.to_str().unwrap()]);
    }

    if link_path.exists() || link_path.is_symlink() {
        fs::remove_file(&link_path).expect("failed to remove existing symlink");
    }

    let skill_src = src_dir.join("skills").join(PLUGIN_NAME);
    unix_fs::symlink(&skill_src, &link_path).expect("failed to create symlink");
    println!("Done. Restart Claude Code to activate the '{}' skill.", PLUGIN_NAME);
}

fn run(cmd: &str, args: &[&str]) {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .unwrap_or_else(|_| { eprintln!("failed to run: {} {:?}", cmd, args); exit(1); });

    if !status.success() {
        eprintln!("{} failed", cmd);
        exit(1);
    }
}
