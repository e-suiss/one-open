//! The export vectors in `export/vectors/` run through the reader.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    reason = "tests"
)]

use std::path::Path;

use one_export_reader::record::{MemoryRecord, RelationshipEdge};
use one_export_reader::{ExportDocument, from_str, to_value};
use serde_json::{Value, json};

fn vectors(name: &str) -> Vec<Value> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../vectors")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap();
    serde_json::from_str::<Value>(&text)
        .unwrap()
        .as_array()
        .unwrap()
        .clone()
}

fn read(document: &Value) -> Option<ExportDocument> {
    from_str(&serde_json::to_string(document).unwrap()).ok()
}

#[test]
fn schema_vectors_accept_and_reject_as_expected() {
    let all = vectors("export-schema.json");
    assert_eq!(all.len(), 19);
    for v in &all {
        assert_eq!(v["input"]["cddl"], json!("export.cddl"));
        assert_eq!(v["input"]["root"], json!("one-export"));
        let valid = read(&v["input"]["document"]).is_some();
        assert_eq!(
            json!({ "valid": valid }),
            v["expected"],
            "{}",
            v["description"]
        );
    }
}

#[test]
fn round_trip_keeps_every_field() {
    let all = vectors("round-trip.json");
    assert_eq!(all.len(), 4);
    let mut checked = 0;
    for v in all.iter().filter(|v| v["expected"].get("equal").is_some()) {
        let doc = read(&v["input"]["document"]).unwrap_or_else(|| panic!("{}", v["description"]));
        let encoded = to_value(&doc).unwrap();
        let again = read(&encoded).unwrap();
        assert_eq!(again, doc, "{}", v["description"]);
        assert_eq!(
            json!({ "equal": encoded == v["input"]["document"], "document": encoded }),
            v["expected"],
            "{}",
            v["description"]
        );
        checked += 1;
    }
    assert_eq!(checked, 3);
}

fn non_ascii_strings(v: &Value, out: &mut Vec<String>) {
    match v {
        Value::String(s) if !s.is_ascii() => out.push(s.clone()),
        Value::Array(items) => items.iter().for_each(|i| non_ascii_strings(i, out)),
        Value::Object(map) => map.values().for_each(|i| non_ascii_strings(i, out)),
        _ => {}
    }
}

#[test]
fn turkish_text_is_kept_byte_for_byte() {
    let v = &vectors("round-trip.json")[2];
    let doc = read(&v["input"]["document"]).unwrap();
    let encoded = serde_json::to_string(&doc).unwrap();
    let mut texts = Vec::new();
    non_ascii_strings(&v["input"]["document"], &mut texts);
    assert_ne!(texts, Vec::<String>::new());
    for t in texts {
        assert!(encoded.contains(&t), "{t}");
    }
}

#[test]
fn imported_originals_are_the_records_as_read() {
    let all = vectors("round-trip.json");
    let v = all
        .iter()
        .find(|v| v["expected"].get("imported").is_some())
        .unwrap();
    let doc = read(&v["input"]["document"]).unwrap();
    let mut live = Vec::new();
    for section in std::iter::once(&doc.memory).chain(doc.sensitive.iter()) {
        for r in &section.records {
            if let MemoryRecord::Live(l) = r {
                live.push(serde_json::to_value(l).unwrap());
            }
        }
        for e in &section.relationship_edges {
            if let RelationshipEdge::Live(l) = e {
                live.push(serde_json::to_value(l).unwrap());
            }
        }
    }
    let originals: Vec<Value> = v["expected"]["imported"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["original"].clone())
        .collect();
    assert_eq!(live, originals);
}

#[test]
fn unknown_fields_are_skipped() {
    let all = vectors("unknown-fields.json");
    assert_eq!(all.len(), 4);
    for v in &all {
        let actual = match read(&v["input"]["document"]) {
            Some(doc) => json!({ "valid": true, "read": to_value(&doc).unwrap() }),
            None => json!({ "valid": false }),
        };
        assert_eq!(actual, v["expected"], "{}", v["description"]);
    }
}

#[test]
fn errors_name_the_problem() {
    let err = from_str("{\"format\":\"zip\"}").unwrap_err();
    assert!(err.to_string().contains("not a valid export package"));
    assert!(from_str("not json").is_err());
}
