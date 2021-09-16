use std::fs;

use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn cli_is_callable() {
    let mut cmd = Command::cargo_bin("holium-sdk-cli").unwrap();
    cmd.arg("--version").assert().success();
}
