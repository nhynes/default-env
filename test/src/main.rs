#![feature(proc_macro_hygiene)]

use default_env::default_env;

fn main() {
    assert_eq!(
        default_env!("DOESNT_EXIST", concat!("default ", "value")),
        "default value"
    );

    assert_eq!(default_env!("CARGO_PKG_NAME", "uh oh"), "default-env-test");
}
