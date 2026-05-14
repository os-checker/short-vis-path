use expect_test::expect_file;
use std::{process::Command, sync::LazyLock};

macro_rules! snapshot {
    ($name:ident) => {
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
        }
    };
    ($($name:ident),+ $(,)?) => {
        $(snapshot! { $name })+
    };
}

snapshot! { usage, ok_single_ident }

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
