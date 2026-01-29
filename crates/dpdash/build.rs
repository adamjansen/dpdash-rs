use std::process::Command;

fn main() {
    // Populate git hash variable if git is available
    println!("cargo:rerun-if-changed=../../.git/logs/HEAD");
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output()
        && output.status.success()
    {
        let git_sha = String::from_utf8_lossy(&output.stdout);
        let git_sha = git_sha.trim();

        let dirty = match Command::new("git")
            .args(["diff-index", "--quiet", "HEAD", "--"])
            .output()
        {
            Ok(_) => output.status.success(),
            _ => false,
        };

        println!(
            "cargo:rustc-env=DPDASH_COMMIT_SHA={}{}",
            git_sha,
            if dirty { "-dirty" } else { "" }
        );
    }
}
