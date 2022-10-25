use crate::{
    chem_tree::{ChemTree, ChemTreeNode},
    chemicals::{Chemical, Data, Reaction},
    local::{serialize, serialize_to_sql, deserialize_from_sql},
    parser,
    search_engine::{Maps}, sql::{setup_database, database}
};
use std::collections::HashMap;

pub fn initialize_compound_tree(optional_path: Option<String>) -> (Box<HashMap<String, ChemTree>>, Maps) {
    match optional_path {
        Some(path) => {
            setup_database(database());
            let reactions = parser::parse(path);
            println!("There are {} compounds.", reactions.len());
            serialize_to_sql(reactions);
        }
        None => {}
    }
    let reactions = deserialize_from_sql();
    let mut reaction_map: HashMap<String, Reaction> = HashMap::with_capacity(reactions.len());
    let mut result_map: HashMap<String, Vec<String>> = HashMap::with_capacity(reactions.len());
    let mut uses_map: HashMap<String, Vec<String>> = HashMap::with_capacity(reactions.len());
    // registers all possible results with their respective internal names
    for reaction in &reactions {
        result_map
            .entry(reaction.get_result())
            .or_default()
            .push(reaction.get_internal_name());
        reaction_map.insert(reaction.get_internal_name(), reaction.clone());

        for recipe in reaction.get_all_recipes() {
            for reagent in recipe {
                let name = reagent.name.clone();
                match uses_map.get(&name) {
                    None => {
                        uses_map
                            .entry(name)
                            .or_default()
                            .push(reaction.get_internal_name());
                    }
                    Some(result) => {
                        if !result.contains(&name) {
                            uses_map
                                .entry(name)
                                .or_default()
                                .push(reaction.get_internal_name());
                        }
                    }
                }
            }
        }
    }

    let maps = Maps {
        reaction_map,
        result_map,
        uses_map,
    };

    let mut compound_trees: Box<HashMap<String, ChemTree>> =
        Box::new(HashMap::with_capacity(reactions.len()));

    for reaction in reactions {
        let name = reaction.get_internal_name();
        let node = ChemTreeNode::new(
            reaction.get_specific_recipe_result_amount(0),
            Chemical::Compound(reaction),
            None,
        );
        let mut chem_tree = ChemTree::new(node);
        chem_tree.populate(&maps);
        compound_trees.insert(name, chem_tree);
    }
    (compound_trees, maps)
}
