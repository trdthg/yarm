use assert_cmd::prelude::*;
use predicates::str::{contains, is_empty};
use std::fs::{self, File};
use std::process::Command;
use tempfile::TempDir;

const TEST_BIN: &str = "yarm";


#[test]
fn no_args() {
    let temp_dir = TempDir::new().unwrap();
    Command::cargo_bin(TEST_BIN)
        .unwrap()
        .current_dir(&temp_dir)
        .assert()
        .failure();
}

#[test]
fn rm_paths() {
    // let temp_dir = TempDir::new().unwrap();
    // let dirpath = temp_dir.path()
    // let files =  [0..100].iter().map(|i| {
    //     let file = tempfile::tempfile_in(&temp_dir).unwrap();

    //     let metadata = file.metadata().unwrap();
    //     file
    // }).collect::<Vec<File>>();
    // Command::cargo_bin(TEST_BIN)
    //     .unwrap()
    //     .args(&["set"])
    //     .current_dir(&temp_dir)
    //     .assert()
    //     .success()
    //     .stdout(is_empty());
}
