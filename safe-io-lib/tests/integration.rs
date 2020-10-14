use tempfile::TempDir;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use safe_io_lib::*;

#[test]
fn read_target_only() {
    let temp_dir = TempDir::new().unwrap();
    let temp = temp_dir.path().to_path_buf();

    let test_content = "test content\n";

    let target = temp.join("test.txt");
    create_file(&target, test_content);

    let content = read_file(target).unwrap();

    assert_eq!(test_content, content);
}

fn create_file(path: &Path, content: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.sync_all().unwrap();
}
