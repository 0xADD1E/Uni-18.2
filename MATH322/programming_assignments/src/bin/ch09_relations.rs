extern crate programming_assignments;
fn main() {
    let filename = programming_assignments::get_argv1().unwrap();
    let filepath = std::path::Path::new(&filename);
    programming_assignments::ch09_relations(&filepath).unwrap();
}
