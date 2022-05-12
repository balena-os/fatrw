use md5::{Digest, Md5};
use tempfile::TempDir;

use std::fs::{read, File};
use std::io::prelude::*;
use std::path::Path;

use fatrw::*;

#[test]
fn read_target_only() {
    let temp_dir = TempDir::new().unwrap();
    let temp = temp_dir.path().to_path_buf();

    let test_content = "test content".as_bytes();

    let target = temp.join("test.txt");
    create_file(&target, test_content);

    let content = read_file(target).unwrap();

    assert_eq!(test_content, content);
}

#[test]
fn read_md5sum() {
    let temp_dir = TempDir::new().unwrap();
    let temp = temp_dir.path().to_path_buf();

    let test_md5sum_content = "test md5sum content".as_bytes();
    let test_target_content = "test target content".as_bytes();

    let target = temp.join("test.txt");
    create_file(&target, test_target_content);

    let checksum = md5sum(test_md5sum_content);

    let md5sum_name = format!(".test.txt.1234abcd.{}.md5sum", checksum);
    let md5sum_path = temp.join(&md5sum_name);
    create_file(&md5sum_path, test_md5sum_content);

    let tmp_name = format!(".test.txt.1234abcd.{}.tmp", checksum);
    let tmp_path = temp.join(&tmp_name);
    create_file(&tmp_path, test_md5sum_content);

    let content = read_file(&target).unwrap();

    let committed_content = read(&target).unwrap();

    assert_eq!(test_md5sum_content, content);
    assert_eq!(committed_content, content);
    assert!(!md5sum_path.exists());
    assert!(!tmp_path.exists());
}

#[test]
fn read_multiple_md5sums() {
    let temp_dir = TempDir::new().unwrap();
    let temp = temp_dir.path().to_path_buf();

    let test_content = "test md5sum content".as_bytes();

    let checksum = md5sum(test_content);

    let md5sum_name_1 = format!(".test.txt.11111111.{}.md5sum", checksum);
    let md5sum_path_1 = temp.join(&md5sum_name_1);
    create_file(&md5sum_path_1, test_content);

    let md5sum_name_2 = format!(".test.txt.22222222.{}.md5sum", checksum);
    let md5sum_path_2 = temp.join(&md5sum_name_2);
    create_file(&md5sum_path_2, test_content);

    let target = temp.join("test.txt");
    let content = read_file(&target).unwrap();

    let committed_content = read(&target).unwrap();

    assert_eq!(test_content, content);
    assert_eq!(committed_content, content);
    assert!(!md5sum_path_1.exists());
    assert!(!md5sum_path_2.exists());
}

fn create_file(path: &Path, content: &[u8]) {
    let mut file = File::create(path).unwrap();
    file.write_all(content).unwrap();
    file.sync_all().unwrap();
}

pub fn md5sum(content: &[u8]) -> String {
    format!("{:x}", Md5::digest(content))
}
