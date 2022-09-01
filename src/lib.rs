extern crate schemafy_core;
extern crate serde;
extern crate serde_json;

pub mod v2 {
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!("src/schema-v2.json");
}

pub mod v3 {
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!("src/schema-v3.json");
}
