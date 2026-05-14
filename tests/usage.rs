use pretty_assertions::assert_eq;
use std::{io::Write, process::Command, sync::LazyLock};

static PWD: LazyLock<String> = LazyLock::new(|| {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
});

fn strip_info(s: &str) -> String {
    s.replace(&**PWD, "$PWD")
}

fn write_output(s: &str, dir: &str, fname: &str) {
    let path = std::path::PathBuf::from_iter([dir, fname]);
    if path.exists() {
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, s, "{dir}/{fname} doesn't match with output");
    } else {
        let mut file = std::fs::File::create_new(&path).unwrap();
        file.write_all(s.as_bytes()).unwrap();
    };
}

#[test]
fn usage() {
    let dir = "usage";
    let output = Command::new("cargo")
        .arg("expand")
        .current_dir(dir)
        .output()
        .unwrap();

    let stdout = strip_info(std::str::from_utf8(&output.stdout).unwrap());
    write_output(&stdout, dir, "stdout.txt");
    let stderr = strip_info(std::str::from_utf8(&output.stderr).unwrap());
    write_output(&stderr, dir, "stderr.txt");
    let status = output.status.success();

    println!("success={status}\nstdout={stdout}\nstderr={stderr}");
}
