use expect_test::expect_file;
use std::{io::Write, process::Command, sync::LazyLock};

static PWD: LazyLock<String> = LazyLock::new(|| {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
});

fn strip_pwd(s: &str) -> String {
    s.replace(&**PWD, "$PWD")
}

#[test]
fn usage() {
    let dir = "usage";
    let output = Command::new("cargo")
        .arg("expand")
        .current_dir(format!("tests/{dir}"))
        .output()
        .unwrap();

    let stdout = strip_pwd(std::str::from_utf8(&output.stdout).unwrap());
    // write_output(&stdout, dir, "stdout.txt");
    expect_file![format!("{dir}/stdout.txt")].assert_eq(&stdout);
    let stderr = strip_pwd(std::str::from_utf8(&output.stderr).unwrap());
    // write_output(&stderr, dir, "stderr.txt");
    expect_file![format!("{dir}/stderr.txt")].assert_eq(&stderr);
    let status = output.status.success();

    println!("success={status}\nstdout={stdout}\nstderr={stderr}");
}
