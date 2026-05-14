use expect_test::expect_file;
use std::{process::Command, sync::LazyLock};

macro_rules! snapshot {
    ($name:ident $success:literal) => {
        #[test]
        fn $name() {
            let dir = stringify!($name);
            let output = Command::new("cargo")
                .arg("expand")
                .current_dir(format!("tests/{dir}"))
                .output()
                .unwrap();

            let stdout = strip_pwd(std::str::from_utf8(&output.stdout).unwrap());
            expect_file![format!("{dir}/cargo_expand.rs")].assert_eq(&stdout);

            // Stdout and stderr are redirected to user.
            let status = Command::new("cargo")
                .arg("check")
                .current_dir(format!("tests/{dir}"))
                .status()
                .unwrap();
            assert_eq!(status.success(), $success);
        }
    };
    (@fail $($name:ident),+ $(,)?) => {
        $(snapshot! { $name false })+
    };
    (@success $($name:ident),+ $(,)?) => {
        $(snapshot! { $name true })+
    };
}

snapshot! { @fail err_single_ident }
snapshot! { @success ok_single_ident }

// Don't include local path in output.
fn strip_pwd(s: &str) -> String {
    static PWD: LazyLock<String> = LazyLock::new(|| {
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    });

    s.replace(&**PWD, "$PWD")
}
