use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead; 

trait Loadable {
    fn load(strings: Vec::<&String>) -> bool;
}

struct Item {
    id: String,
    count: i32,
}

impl Item {
    fn new(id: String) -> Item {
        Item{
            id,
            count: 0
        }
    }
}

enum VarType {
    VTInteger(i32),
    VTBool(bool),
    VTString(String)
}

struct Var {
    id: String,
    var_type: VarType 
}

impl Var {
    fn from(line: &String, line_number: usize) -> Var {

        // get variable name
        let mut line_parsed: Vec<&str> = line.trim().split(':').collect();
        let var_name = line_parsed[0].trim();
        
        //get variable type
        line_parsed = line_parsed[1].trim().split("=").collect();
        let var_type = line_parsed[0].trim();

        //get variable default value
        line_parsed = line_parsed[1].trim().split("@").collect();
        let var_default = line_parsed[0].trim();
        let _var_other = match line_parsed.get(1){ // TODO implement var_other
            Some(string) => string,
            None => ""
        };

        //println!("name: {}, type: {}, default: {}, other = {}", var_name, var_type, var_default, _var_other);


        // Match var_type        
        let var_type_struct: VarType = match var_type {
            "int" => {
                let var_default_typed : i32 = match var_default.parse() {
                    Ok(value) => value,
                    Err(_) => panic!("Couldn't load default value( {} ) of var {} as int at line {}", var_default, var_name, line_number)
                };
                VarType::VTInteger(var_default_typed)
            },

            "str" => {
                let var_default_typed : String = String::from(var_default);
                VarType::VTString(var_default_typed)
            },

            "bool" => {
                let var_default_typed : bool = match var_default {
                    "true" => true,
                    "false" => false,
                    _ => panic!("undefined bool parameter at line {}", line_number)
                };
                VarType::VTBool(var_default_typed)
            }

            _ => {
                panic!("Unknown variable type <{}> at line {}", var_type, line_number)
            }
        };


        Var {
            id: String::from(var_name),
            var_type: var_type_struct
        }

    }
}

struct OptionCommand {
    keyword: String,
    argument_count: i32, // -1 -> non defined
    run_fn: fn(Vec<String>, &Adventure)
}

struct SceneOption {
    id: String,
    text: String,
    run: Vec<OptionCommand>
}

struct Scene {
    id: String,
    name: String,
    options: Vec<Vec<SceneOption>> // one vector for mutliple statement lines, second for commands itself
}

struct Adventure {
    version: String, 

    items: HashMap<String, Item>,
    vars: HashMap<String, Var>,
    scenes: HashMap<String, Scene>,
    current_scene: Option<Scene>
}

fn trim_comment(line: String) -> String {
    /*
    Trim comments on end
    */
    let comment_split: Vec<&str> = line.split('#').collect();
    String::from(comment_split[0])
}

impl Adventure {
    fn new() -> Adventure {
        Adventure {
            version : String::from("--- ADVENTURE FORMAT  [0.1] ---"),

            items: HashMap::new(),
            vars: HashMap::new(),
            scenes: HashMap::new(),

            current_scene: None
        }
    }

    fn load_from_file(&mut self, path: &String) -> std::io::Result<()> {
        let file = OpenOptions::new().read(true).open(path)?;

        let buff = BufReader::new(file);

        let mut section_buffer :Vec<String> = Vec::<String>::new();
        for (line_number, line) in buff.lines().enumerate() {
            let line_uw = trim_comment(line?);

            // Ignore first line
            if line_number == 0 { 
                if line_uw != self.version {
                    panic!(" Bad version of Adventure Format");
                }

                continue;
            }
            
            // If section ends (section ends on "---")
            if line_uw.trim() == "---" {
                self.load_section(&mut section_buffer, line_number);
                section_buffer.clear();
                continue;
            };
            section_buffer.push(line_uw);
        }

        Ok(())
    }

    fn load_section(&mut self, section_lines : &mut Vec<String>,mut line_number: usize) {
        if section_lines.len() == 0 {
            panic!("Unvalid section on line {}", line_number);
        }

        let name: String = section_lines.remove(0);
        
        // Set line_number to start of section not end
        line_number = line_number - section_lines.len() + 1;

        match name.trim() {
            "ITEMS:" => self.handle_items_section(section_lines, line_number),
            "VARS:" => self.handle_vars_section(section_lines, line_number),
            "SCENE:" => self.handle_scene_section(section_lines, line_number),
            _ => panic!("No handler for {}", name.trim())
        };

        

    }

    fn handle_items_section(&mut self, section_lines : &Vec<String>, line_number: usize) {
        for (line_offset, line) in section_lines.iter().enumerate() {
            let item_name = line.trim();
            match self.items.get(item_name) {
                Some(_) => panic!("Ite with name {} appeared repeately at line {}", item_name, line_number + line_offset),
                None => self.items.insert(item_name.to_string(), Item::new(item_name.to_string()))
            };

            
        }
    }

    fn parse_var(&mut self, line: &String, line_number: usize) {
        if line.trim() == "" {
            return
        }
        
        // get variable name
        let mut line_parsed: Vec<&str> = line.trim().split(':').collect();
        let var_name = line_parsed[0].trim();

        // check if variable with the same name already do not exists

        match self.vars.get(var_name) {
            Some(_) => panic!("There is already variable with name {}", var_name),
            _ => ()
        }

        //create variable

        self.vars.insert(String::from(var_name), Var::from(line, line_number));
    }

    fn handle_vars_section(&mut self, section_lines : &Vec<String>, line_number: usize) {
        for (line_offset, line) in section_lines.iter().enumerate() {
            self.parse_var(&line, line_number + line_offset);
        }
    }

    fn handle_scene_section(&self, section_lines : &Vec<String>, line_number: usize) {

    }


}



fn main() -> std::io::Result<()> {

    let mut adventure = Adventure::new();

    adventure.load_from_file(&String::from("adventure_demo.av"))?;


    Ok(())


}
