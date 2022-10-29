use crate::{
    local::serialize_to_sql,
    parser, sql::{setup_database, database}
};

pub fn initialize_compound_tree(optional_path: Option<String>) {
    match optional_path {
        Some(path) => {
            setup_database(database());
            let reactions = parser::parse(path);
            println!("There are {} compounds.", reactions.len());
            serialize_to_sql(reactions);
        }
        None => {}
    }
}