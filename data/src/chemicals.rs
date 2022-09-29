use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;

// static BASES: [Base; 30] = [
//     Base { id: "aluminium" },
//     Base { id: "barium" },
//     Base { id: "bromine" },
//     Base { id: "calcium" },
//     Base { id: "carbon" },
//     Base { id: "chlorine" },
//     Base { id: "chromium" },
//     Base { id: "copper" },
//     Base { id: "ethanol" },
//     Base { id: "fluorine" },
//     Base { id: "hydrogen" },
//     Base { id: "iodine" },
//     Base { id: "iron" },
//     Base { id: "lithium" },
//     Base { id: "magnesium" },
//     Base { id: "mercury" },
//     Base { id: "nickel" },
//     Base { id: "nitrogen" },
//     Base { id: "oxygen" },
//     Base { id: "phosphorus" },
//     Base { id: "plasma" },
//     Base { id: "platinum" },
//     Base { id: "potassium" },
//     Base { id: "radium" },
//     Base { id: "silicon" },
//     Base { id: "silver" },
//     Base { id: "sodium" },
//     Base { id: "sugar" },
//     Base { id: "sulfur" },
//     Base { id: "water" },
// ];

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Base {
    Aluminium,
    Barium,
    Bromine,
    Calcium,
    Carbon,
    Chlorine,
    Chromium,
    Copper,
    Ethanol,
    Fluorine,
    Hydrogen,
    Iodine,
    Iron,
    Lithium,
    Magnesium,
    Mercury,
    Nickel,
    Nitrogen,
    Oxygen,
    Phosphorus,
    Plasma,
    Platinum,
    Potassium,
    Radium,
    Silicon,
    Silver,
    Sodium,
    Sugar,
    Sulfur,
    Water,
}

impl Base {
    pub fn get_id(&self) -> String {
        let id = format!("{:?}", self);
        id.to_lowercase()
    }
}

pub static BASES: [Base; 30] = [
    Base::Aluminium,
    Base::Barium,
    Base::Bromine,
    Base::Calcium,
    Base::Carbon,
    Base::Chlorine,
    Base::Chromium,
    Base::Copper,
    Base::Ethanol,
    Base::Fluorine,
    Base::Hydrogen,
    Base::Iodine,
    Base::Iron,
    Base::Lithium,
    Base::Magnesium,
    Base::Mercury,
    Base::Nickel,
    Base::Nitrogen,
    Base::Oxygen,
    Base::Phosphorus,
    Base::Plasma,
    Base::Platinum,
    Base::Potassium,
    Base::Radium,
    Base::Silicon,
    Base::Silver,
    Base::Sodium,
    Base::Sugar,
    Base::Sulfur,
    Base::Water,
];

pub static BASES_MAP: Lazy<HashMap<&str, Base>> = Lazy::new(|| {
    HashMap::from([
    ("aluminium", Base::Aluminium),
    ("barium", Base::Barium),
    ("bromine", Base::Bromine),
    ("calcium", Base::Calcium),
    ("carbon", Base::Carbon),
    ("chlorine", Base::Chlorine),
    ("chromium", Base::Chromium),
    ("copper", Base::Copper),
    ("ethanol", Base::Ethanol),
    ("fluorine", Base::Fluorine),
    ("hydrogen", Base::Hydrogen),
    ("iodine", Base::Iodine),
    ("iron", Base::Iron),
    ("lithium", Base::Lithium),
    ("magnesium", Base::Magnesium),
    ("mercury", Base::Mercury),
    ("nickel", Base::Nickel),
    ("nitrogen", Base::Nitrogen),
    ("oxygen", Base::Oxygen),
    ("phosphorus", Base::Phosphorus),
    ("plasma", Base::Plasma),
    ("platinum", Base::Platinum),
    ("potassium", Base::Potassium),
    ("radium", Base::Radium),
    ("silicon", Base::Silicon),
    ("silver", Base::Silver),
    ("sodium", Base::Sodium),
    ("sugar", Base::Sugar),
    ("sulfur", Base::Sulfur),
    ("water", Base::Water),])
});

// Finding all of these will be difficult
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Ingredient{
    id: String,
}

impl Ingredient {
    pub fn new(id: String) -> Ingredient{
        Ingredient { id }
    }

    pub fn get_id(&self) -> String {
        let id = format!("{}", self.id);
        id.to_lowercase()
    }
}



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawReagent {
    name: String,
    quantity: u32,
}

impl RawReagent {
    pub fn new(name: String, quantity: u32) -> RawReagent {
        RawReagent { name, quantity }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reagent {
    chemical: Chemical,
    quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Data {
    pub compounds: Vec<Compound>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Chemical {
    Base(Base),
    Compound(Compound),
    Ingredient(Ingredient),
}

impl Chemical {
    pub fn get_id(&self) -> String {
        match self {
            Chemical::Base(base) => base.get_id(),
            Chemical::Ingredient(ingredient) => ingredient.get_id(),
            Chemical::Compound(compound) => compound.get_id(),

        }
    }   
}

#[derive(Debug)]
pub struct ChemTree{
    root: Box<ChemTreeNode>,
}

impl ChemTree {
    pub fn new(root: ChemTreeNode) -> ChemTree{
        ChemTree{
            root: Box::new(root),
        }
    }

    pub fn print_dispenser_format(&self){
        println!("\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\");
        println!("----\t{}\t----\n", self.root.get_id().to_uppercase());


        let mut pastable_string = String::new();
        let mut compounds = String::new();
        let mut ingredients = String::new();
        
        for node in self.root.get_reagents() {
            for reagent in node{
                let result = reagent.print_branch(0);
                match result.0{
                    Chemical::Compound(_compound) => {
                        compounds = format!("{}\n{}", compounds, result.1.as_str());
                    }
                    Chemical::Base(_base) => {
                        pastable_string.push_str(result.1.as_str());
                    }
                    Chemical::Ingredient(_ingredient) => {
                        ingredients.push_str(result.1.as_str());
                    }
                }
            }
        }

        if !compounds.is_empty() || !ingredients.is_empty(){
            println!("# Non-base Reagents #");
            println!("+++++++++++++++++++++++++++++++++++++");
    
            if !compounds.is_empty(){
    
                println!("_compounds_");
                println!("{}", compounds);
            }
    
            if !ingredients.is_empty(){
                println!("_ingredients_");
                println!("{}", ingredients);
            }
            println!("+++++++++++++++++++++++++++++++++++++\n");
    
        }
        
        if !pastable_string.is_empty(){
            println!("# Base Reagents #");
            println!("-------------------------------------");
            println!("{}", pastable_string);
            println!("-------------------------------------");
        }

        println!("////////////////////////////////////\n");


    }

    pub fn populate(&mut self, compound_map: &HashMap<String, Compound>){

        let id = self.root.get_id();
        let chem = Chemical::Compound(compound_map.get(&id).unwrap().clone());
        
        let branches = Self::populate_branches(chem, compound_map);

        self.root.push_root_branches(branches);
    }

    fn populate_branches(chem: Chemical, compound_map: &HashMap<String, Compound>) -> Vec<ChemTreeNode>{
        let id = chem.get_id();
        let raw_reagents = compound_map.get(&id).unwrap().get_reagents();
        let mut branches: Vec<ChemTreeNode> = Vec::new();
        

        for reagent in raw_reagents{
            let mut reagents: Option<Vec<ChemTreeNode>> = None;
                        let chemical: Chemical;
            let name = &reagent.name;
            let quantity = reagent.quantity;

            if compound_map.contains_key(name){
                chemical = Chemical::Compound(compound_map.get(name).unwrap().clone());
                reagents = Some(Self::populate_branches(chemical.clone(), &compound_map));
            }else if BASES_MAP.contains_key(&name.as_str()){
                chemical = Chemical::Base(BASES_MAP.get(&name.as_str()).unwrap().clone());
            }else{
                chemical = Chemical::Ingredient(Ingredient::new(name.clone()));
            }

            let reagent_node = ChemTreeNode::new(
                quantity as f32,
                chemical,
                reagents
            );

            branches.push(reagent_node);
        }

        branches
    }
}

#[derive(Debug)]
pub struct ChemTreeNode{
    chemical: Chemical,
    quantity: f32,
    reagents: Box<Option<Vec<ChemTreeNode>>>
}

impl ChemTreeNode {
    pub fn get_id(&self) -> String{
        self.chemical.get_id()
    }
    
    fn push_root_branches(&mut self, branches: Vec<ChemTreeNode>){
        self.reagents = Box::new(Some(branches));
    }

    fn get_reagents(&self) -> &Option<Vec<ChemTreeNode>>{
        &self.reagents
    }

    // probably needs to be broken into seperate functions for each reagent type
    fn print_branch(&self, layer: i8) -> (&Chemical, String) {
        let result:(&Chemical, String);

        let mut tab = String::new();
        let mut c = layer;
        while c > 0{
            tab = format!("\t{}", tab);
            c -= 1;
        }

        match &self.chemical{
            Chemical::Compound(compound) => {

                let mut branch_strings = Vec::new();
                for vec in self.get_reagents(){
                    for node in vec{
                        branch_strings.push(node.print_branch(layer + 1));
                    }
                }

                let mut pastable_string = String::new();
                let mut compounds = String::new();
                let mut ingredients = String::new();

                for s in branch_strings{
                    match s.0{
                        Chemical::Compound(_compound) => {
                            compounds = format!("{}\n{}", compounds, s.1.as_str());
                        }
                        Chemical::Base(_base) => {
                            pastable_string.push_str(s.1.as_str());
                        }
                        Chemical::Ingredient(_ingredient) => {
                            ingredients.push_str(s.1.as_str());
                        }
                    }
                }

                let mut branch = String::new();
                if !pastable_string.is_empty(){
                    branch = format!("\n{tab}\t{pastable_string}");
                }
                if !ingredients.is_empty(){
                    branch = format!("{branch}\n{tab}\t{ingredients}");
                }
                if !compounds.is_empty(){
                    branch = format!("{branch}\n{tab}{compounds}");
                }

                let compound_value = format!("{tab}{} {}", self.quantity, compound.get_id().to_uppercase());

                let recipe  = format!("{}\n{tab}[\n{}\n{tab}]\n", compound_value, branch);

                result = (&self.chemical, recipe);
            }
            Chemical::Base(base) => {
                result = (&self.chemical, format!("{}={};", base.get_id(), self.quantity));
            }
            Chemical::Ingredient(ingredient) => {
                result = (&self.chemical, format!("[{} {}]", self.quantity, ingredient.get_id()));
            }
        }

        result

    }
}

impl ChemTreeNode {
    pub fn new(quantity: f32, chemical: Chemical, reagents: Option<Vec<ChemTreeNode>>) -> ChemTreeNode{
        ChemTreeNode { chemical, quantity,  reagents: Box::new(reagents) }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Compound {
    internal_name: String,
    name: String,
    id: String,
    result: String,
    mix_phrase: String,
    raw_reagents: Vec<RawReagent>,
    required_reagents: Vec<Reagent>,
    result_amount: f32,
    hidden: Option<bool>,
}

impl Compound {
    pub fn new(
        internal_name: String,
        name: String,
        id: String,
        result: String,
        mix_phrase: String,
        raw_reagents: Vec<RawReagent>,
        required_reagents: Vec<Reagent>,
        result_amount: f32,
        hidden: Option<bool>,
    ) -> Compound {
        Compound {
            internal_name,
            name,
            id,
            result,
            mix_phrase,
            raw_reagents,
            required_reagents,
            result_amount,
            hidden,
        }
    }

    //if problems occur change this to get result
    pub fn get_id(&self) -> String{
        self.id.clone()
    }

    pub fn get_result_amount(&self) -> f32 {
        self.result_amount.clone()
    }
    
    pub fn get_reagents(&self) -> &Vec<RawReagent> {
        &self.raw_reagents
    }


}