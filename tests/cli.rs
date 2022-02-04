use assert_cmd::prelude::*;
use predicates::str::{contains, is_empty};
use std::fs::{self, File};
use std::process::Command;
use tempfile::TempDir;

const TEST_BIN: &str = "yarm";

/// rm 失败
/// rm a.txt b.py c.c 只删除文件        <Path>...
/// rm folder a.txt  删除文件和文件夹
/// rm *.txt 删除当前文件夹下的匹配文件
/// rm folder/*  删除文件夹下的东西
/// rm folder1/* folder2/* a.txt
///
/// rm folder -e a.txt  指定文件夹删除除了某些文件
/// rm -e a.txt  不指定文件夹(默认当前文件夹)

#[test]
fn client_cli_no_args() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin(TEST_BIN).unwrap();
    cmd.current_dir(&temp_dir).assert().failure();
}
