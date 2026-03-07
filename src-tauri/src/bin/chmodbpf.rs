use std::{fs, os::unix::fs::PermissionsExt, process::Command};

fn main() {
    let Ok(entries) = fs::read_dir("/dev") else {
        eprintln!("chmodbpf: failed to read /dev");
        std::process::exit(1);
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let is_bpf = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("bpf"))
            .unwrap_or(false);

        if is_bpf {
            let path_str = match path.to_str() {
                Some(s) => s,
                None => continue,
            };

            let status = Command::new("/usr/sbin/chown")
                .args(["root:admin", path_str])
                .status();

            if let Err(e) = status {
                eprintln!("chmodbpf: chown failed for {}: {}", path_str, e);
            }

            if let Err(e) = fs::set_permissions(&path, PermissionsExt::from_mode(0o640)) {
                eprintln!("chmodbpf: chmod failed for {}: {}", path_str, e);
            }
        }
    }
}
