extern crate schemafy_core;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

schemafy::schemafy!("src/schema.json");
