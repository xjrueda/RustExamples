use std::{collections::HashMap, path::PathBuf};
use dsync::{GenerationConfig, TableOptions};

pub fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");
    dsync::generate_files(
        PathBuf::from_iter([dir, "src/schema.rs"]),
        PathBuf::from_iter([dir, "src/models"]),
        GenerationConfig {
            default_table_options: TableOptions::default(),
            table_options: HashMap::from([]),
            connection_type: "PooledConnection<PgConnection>".to_string()
        }
    );
}