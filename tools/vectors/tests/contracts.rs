//! Every schema in the repository compiles and every schema vector gives its expected result.

#![allow(clippy::unwrap_used, reason = "tests")]

use std::path::Path;

#[test]
fn schemas_compile_and_schema_vectors_hold() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let report = one_open_vectors::check(&root).unwrap();
    assert_eq!(report.problems, Vec::<String>::new());
    assert_eq!(report.schemas, 25);
    assert_eq!(report.vectors, 172);
}
