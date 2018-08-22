extern crate programming_assignments;
extern crate tempfile;

use std::io::Write;

#[test]
fn chapter_9_smoke() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    let symbols = "a, b, c, d";
    let matrix = "{{1, 0, 0, 1}, {1, 1, 0, 0}, {1, 0, 1, 1}, {0, 0, 1, 0}}";

    writeln!(file, "{}", symbols);
    writeln!(file, "{}", matrix);
    programming_assignments::ch09_relations(&file.path()).unwrap();
}
