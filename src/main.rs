use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead; 

trait Loadable {
    fn load(strings: Vec::<&String>) -> bool;
}

#[derive(Debug)]
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

#[derive(Debug)]
enum VarType {
    VTInteger(i32),
    VTBool(bool),
    VTString(String)
}

#[derive(Debug)]
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

/*struct OptionCommand {
    keyword: String,
    argument_count: i32, // -1 -> non defined
    run_fn: fn(Vec<String>, &Adventure)
}*/

#[derive(Debug)]
enum OptionCommand {
    IfHave(String),  // IF
    IfNotHave(String),
    IfLessThan(String, i32),
    IfMoreThan(String, i32),

    Add(String), // ITEMS
    Remove(String),

    Increment(String, i32), // VARS
    Decrement(String, i32),
    Set(String, i32),

    Jump(String),
    End
}


fn parse_option_command(input: String) -> OptionCommand {
    let input_split: Vec<&str> = input.trim().split(" ").collect();

    match input_split[0] {
        "if" => {
            match input_split[1] {
                "have" => OptionCommand::IfHave(input_split[2].to_string()),
                "not_have" => OptionCommand::IfNotHave(input_split[2].to_string()),
                _ => {
                    match input_split[2] {
                        "less_than" => OptionCommand::IfLessThan(input_split[1].to_string(),input_split[3].parse::<i32>().unwrap()),
                        "more_than" => OptionCommand::IfMoreThan(input_split[1].to_string(),input_split[3].parse::<i32>().unwrap()),

                        _ => panic!("There is no if statement type 2 with subcommand {}", input_split[2])
                    }
                }
            }
        }

        "add" => OptionCommand::Add(input_split[1].to_string()),
        "remove" => OptionCommand::Remove(input_split[1].to_string()),

        "increment" => OptionCommand::Increment(input_split[1].to_string(), input_split[2].parse::<i32>().unwrap()),
        "decrement" => OptionCommand::Decrement(input_split[1].to_string(), input_split[2].parse::<i32>().unwrap()),
        "set" => OptionCommand::Set(input_split[1].to_string(), input_split[2].parse::<i32>().unwrap()),

        "jump" => OptionCommand::Jump(input_split[1].to_string()),
        "end" => OptionCommand::End,
        
        _ => panic!("there is no command {}", input_split[0])        
    }



}

#[derive(Debug)]
struct OptionCommandLine {
    commands: Vec<OptionCommand>
}

impl OptionCommandLine {
    fn new() -> OptionCommandLine {
        OptionCommandLine {
            commands: Vec::<_>::new()
        }
    }
    fn push_command(&mut self, command: OptionCommand) {
            self.commands.push(command)
    
        }

    fn from(string :String) -> OptionCommandLine {
        let mut command_line = OptionCommandLine::new();

        let line_split: Vec<&str> = string.trim().split(";").collect();
        
        for line in line_split.iter() {
            command_line.push_command(parse_option_command(line.to_string()))
        }
        command_line

    }
}

#[derive(Debug)]
struct OptionCommandBlock {
    lines : Vec<OptionCommandLine>
}

impl OptionCommandBlock {
    fn new() -> OptionCommandBlock {
        OptionCommandBlock{
            lines: Vec::<_>::new()
        }
    }
    fn from(lines :Vec<String>) -> OptionCommandBlock {
        let mut block = OptionCommandBlock::new();

        for line in lines {
            block.lines.push(OptionCommandLine::from(line));
        }

        block
    }
}

#[derive(Debug)]
struct SceneOption {
    id: usize,
    text: String,
    run: Option<OptionCommandBlock>
}

impl SceneOption {

    fn from(id: usize) -> SceneOption {
        SceneOption {
            id,
            text: String::from(""),
            run: None
        }

    }
}

#[derive(Debug)]
enum SceneInputType {
    SIOneLine(String, String), // Name, rest
    SIMultiLine(String, Vec<String>) // Name, rest
}

#[derive(Debug)]
struct Scene {
    id: String,
    name: String,
    text: String,
    run: Option<OptionCommandBlock>,
    options: Vec<SceneOption> // one vector for mutliple statement lines, second for commands itself
}

impl Scene {
    fn new() -> Scene {
        Scene {
            id: String::from(""),
            name: String::from(""),
            text: String::from(""),
            run: None,
            options: Vec::<_>::new()
        }
    }

    fn handle_scene_input(&mut self, input: SceneInputType, line_number: usize) {
        match input {
            SceneInputType::SIOneLine(first, second) =>{
                match &first[..] { // id, name
                    "id" => self.id = second,
                    "name" => self.name = second,
                    _ => panic!("Unknown scene attribute {} at line {}", first, line_number)

                }
            }

            SceneInputType::SIMultiLine(first, second) => {

                match &first[..] { // text, option, run
                    "text" => {
                        self.text = second.join("\n");

                    }

                    "run" => {
                        self.run = Some(OptionCommandBlock::from(second));
                    }

                    "option" => {
                        self.parse_option(second, line_number);
                    }

                    _ => panic!("Unknown scene attribute {} at line {}", first, line_number)
                }

            }
        }
    }

    fn parse_option(&mut self,lines :Vec<String>, line_number: usize) {
        // TODO ...

        let mut option = SceneOption::from(self.options.len());
        
        let mut in_run = false;
        let mut run_buffer = Vec::<String>::new();

        for (line_offset, line) in lines.iter().enumerate() {
            if in_run{
                run_buffer.push(line.clone());
                continue;
            }

            let mut line_split: Vec<&str> = line.trim().split(":").collect();
            match line_split[0].trim() {
                "text" => {
                    option.text = String::from(line_split[1]);
                }
                "run" => {
                    in_run = true;
                    continue;
                }

                _ => panic!("Unknown option argument {} at line {}", line_split[0], line_number + line_offset)
            }


        }

        if !in_run {
            panic!("There is no run sequence in option at line {}", line_number);
        }

        option.run = Some(OptionCommandBlock::from(run_buffer));

        self.options.push(option);
    }



    fn run(&self) {
        println!("Shit..");
    }
}


#[derive(Debug)]
struct Adventure {
    version: String, 

    items: HashMap<String, Item>,
    vars: HashMap<String, Var>,
    scenes: HashMap<String, Scene>,
    current_scene_id: String
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
            current_scene_id: String::from(""),

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

    fn handle_scene_section(&mut self, section_lines : &Vec<String>, line_number: usize) {
        let mut in_multiline = false; // if multiline starts with empty right side of ':' and ends with blank line
        let mut multiline_buffer : Vec<String> = Vec::<_>::new();
        let mut multiline_name: String = String::from("");

        let mut scene: Scene = Scene::new();

        for (line_offset, line) in section_lines.iter().enumerate() {
            let line_trimmed = line.trim();
            if line_trimmed == "" && !in_multiline {
                continue;
            }

            if in_multiline {
                if line_trimmed == "" {
                    in_multiline = false;
                    scene.handle_scene_input(SceneInputType::SIMultiLine(multiline_name, multiline_buffer), line_number + line_offset);
                    multiline_buffer = Vec::<_>::new();
                    multiline_name = String::from("");
                }
                else {
                    multiline_buffer.push(String::from(line));
                    continue;
                }
            }

            else {
                let line_split: Vec<&str> = line_trimmed.split(":").collect();
                let mut line_split_len = line_split.len();
                if line_split[1] == "" {
                    line_split_len = 1;
                }

                let first_parameter: String = String::from(line_split[0]);

                if line_split_len == 1 {
                    in_multiline = true;
                    multiline_name = String::from(line_split[0]);
                } else if line_split_len == 2 {
                    scene.handle_scene_input(SceneInputType::SIOneLine(first_parameter, String::from(line_split[1])), line_number + line_offset);
                }
                else {
                    panic!("More than 2 chars '.' at line {}", line_number + line_offset);
                }

            }
        }

        // Set first scene id
        if self.current_scene_id == "" {
            self.current_scene_id = scene.id.clone();
        }

        let id_coppied = scene.id.clone();
        

        self.scenes.insert(id_coppied, scene);
    }


    fn run(&mut self) {
        self.scenes[&self.current_scene_id].run();
    }


}



fn main() -> std::io::Result<()> {

    let mut adventure = Adventure::new();

    adventure.load_from_file(&String::from("adventure_demo.av"))?;

    //println!("{:?})", adventure);
    adventure.run();


    Ok(())


}
