use std::process::Command;
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

#[test]
fn no_argument() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("shellborn-cli")?;

    cmd.assert()
        .code(2);

    Ok(())
}

#[test]
fn invalid_argument() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("shellborn-cli")?;

    cmd.arg("doesnt_exit.txt");

    cmd.assert()
        .code(1);

    Ok(())
}

#[test]
fn valid_argument() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("shellborn-cli")?;

    cmd.arg("res/example.txt");

    cmd.assert()
        .code(0);

    Ok(())
}