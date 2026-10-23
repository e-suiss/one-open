//! Contract checks for this repository: every JSON Schema listed in an `index.toml` compiles against
//! the 2020-12 metaschema with every `$ref` resolved inside the repository, its `$id` matches the
//! index, every listed file exists, and every schema vector gives its expected result.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

/// Folders that carry an `index.toml`.
pub const FOLDERS: [&str; 3] = ["schemas", "federation", "capability"];

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Index {
    #[serde(default)]
    schema: Vec<SchemaEntry>,
    #[serde(default)]
    vectors: Vec<VectorEntry>,
    #[serde(default)]
    table: Vec<TableEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SchemaEntry {
    path: String,
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct VectorEntry {
    path: String,
    kind: String,
    #[serde(default)]
    schema: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TableEntry {
    path: String,
}

#[derive(Debug, Deserialize)]
struct SchemaVector {
    description: String,
    input: serde_json::Value,
    valid: bool,
}

/// The outcome of [`check`].
#[derive(Debug, Default)]
pub struct Report {
    /// Schemas compiled.
    pub schemas: usize,
    /// Schema vectors run.
    pub vectors: usize,
    /// Problems found; empty when everything holds.
    pub problems: Vec<String>,
}

fn read_json(path: &Path) -> Result<serde_json::Value, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

fn read_index(root: &Path, folder: &str) -> Result<Index, String> {
    let path = root.join(folder).join("index.toml");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    toml::from_str(&text).map_err(|e| format!("{folder}/index.toml: {e}"))
}

/// Runs every check under `root`, the repository root.
///
/// # Errors
/// A file that cannot be read or parsed.
pub fn check(root: &Path) -> Result<Report, String> {
    let mut report = Report::default();
    let mut indexes = Vec::new();
    for folder in FOLDERS {
        indexes.push((folder, read_index(root, folder)?));
    }
    let mut compiler = boon::Compiler::new();
    let mut ids: BTreeMap<String, String> = BTreeMap::new();
    for (folder, index) in &indexes {
        for s in &index.schema {
            let rel = format!("{folder}/{}", s.path);
            let json = read_json(&root.join(&rel))?;
            if json.get("$id").and_then(serde_json::Value::as_str) != Some(s.id.as_str()) {
                report
                    .problems
                    .push(format!("{rel}: `$id` must be `{}`", s.id));
            }
            if let Some(prev) = ids.insert(s.id.clone(), rel.clone()) {
                report
                    .problems
                    .push(format!("{rel}: `$id` also used by {prev}"));
            }
            if let Err(e) = compiler.add_resource(&s.id, json) {
                report
                    .problems
                    .push(format!("{rel}: cannot load schema: {e}"));
            }
        }
    }
    let mut schemas = boon::Schemas::new();
    let mut by_id: BTreeMap<String, boon::SchemaIndex> = BTreeMap::new();
    for (id, rel) in &ids {
        match compiler.compile(id, &mut schemas) {
            Ok(index) => {
                by_id.insert(id.clone(), index);
                report.schemas = report.schemas.saturating_add(1);
            }
            Err(e) => report
                .problems
                .push(format!("{rel}: schema does not compile: {e:#}")),
        }
    }
    for (folder, index) in &indexes {
        for t in &index.table {
            let rel = format!("{folder}/{}", t.path);
            if !root.join(&rel).is_file() {
                report
                    .problems
                    .push(format!("{rel}: listed table is missing"));
            }
        }
        for entry in &index.vectors {
            let rel = format!("{folder}/{}", entry.path);
            let json = read_json(&root.join(&rel))?;
            match entry.kind.as_str() {
                "behaviour" => {}
                "schema" => {
                    let Some(schema) = entry.schema.as_ref().and_then(|id| by_id.get(id)) else {
                        report
                            .problems
                            .push(format!("{rel}: schema vectors name no compiled schema"));
                        continue;
                    };
                    let vectors: Vec<SchemaVector> =
                        serde_json::from_value(json).map_err(|e| format!("{rel}: {e}"))?;
                    for (i, vector) in vectors.iter().enumerate() {
                        let valid = schemas.validate(&vector.input, *schema).is_ok();
                        report.vectors = report.vectors.saturating_add(1);
                        if valid != vector.valid {
                            report.problems.push(format!(
                                "{rel}#{i}: {}: expected valid={}, got {valid}",
                                vector.description, vector.valid
                            ));
                        }
                    }
                }
                other => report
                    .problems
                    .push(format!("{rel}: unknown vector kind `{other}`")),
            }
        }
    }
    Ok(report)
}
