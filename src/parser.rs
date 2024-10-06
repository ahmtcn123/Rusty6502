use std::collections::{BTreeMap, HashMap};

use crate::asm::{Instructions, RawAddrCode};

#[derive(Debug)]
pub enum AssemblerError {
    InvalidDirectiveLine(usize),
    InvalidDirectiveValue(usize),
    InvalidInstruction(usize),
    InvalidLabel(usize),
    InvalidValue(&'static str),
}

#[derive(Debug, Clone)]
pub struct AssemblerDirective(String, String);

#[derive(Debug, Clone)]
pub struct Label {
    pub title: String,
    pub instructions: Vec<String>,
    pub directives: Vec<AssemblerDirective>,
}

#[derive(Debug, Clone)]
pub struct TokenizeResult(Vec<Label>);

/// Tokenize the program from given string
pub fn tokenize_assembly_string(code: &str) -> Result<TokenizeResult, AssemblerError> {
    //split lines with eol
    let lines = code.lines();
    let mut active_scope = "_".to_string();
    let mut labels: Vec<Label> = Vec::new();
    labels.push(Label {
        title: active_scope.clone(),
        instructions: Vec::new(),
        directives: Vec::new(),
    });

    for (idx, unformated_line) in lines.enumerate() {
        if unformated_line.trim().is_empty() {
            continue;
        }

        //if line starts with \t or 4 empty spaces, it is a scope
        if unformated_line.starts_with("\t") || unformated_line.starts_with("    ") {
            let line = unformated_line.trim().to_string();
            if line.starts_with(".") {
                let parts = line.trim().split(" ").collect::<Vec<_>>();
                if parts.len() != 2 {
                    return Err(AssemblerError::InvalidDirectiveLine(idx + 1));
                }

                let directive = parts[0].trim().replace(".", "");
                let value = parts[1].trim();
                labels
                    .last_mut()
                    .unwrap()
                    .directives
                    .push(AssemblerDirective(directive.to_string(), value.to_string()));
            } else {
                let mut formated_line = line.trim().to_string();
                if line.contains(";") {
                    formated_line = formated_line.split(";").collect::<Vec<_>>()[0].to_string();
                }
                labels.last_mut().unwrap().instructions.push(formated_line);
            }
        } else {
            if unformated_line.contains(":") {
                let parts = unformated_line.split(":").collect::<Vec<_>>();
                if parts.len() != 2 {
                    return Err(AssemblerError::InvalidLabel(idx + 1));
                }
                let label = parts[0].trim();
                if label.is_empty() {
                    return Err(AssemblerError::InvalidLabel(idx + 1));
                }

                active_scope = label.to_string();

                labels.push(Label {
                    title: active_scope.clone(),
                    instructions: Vec::new(),
                    directives: Vec::new(),
                });
            } else {
                return Err(AssemblerError::InvalidLabel(idx + 1));
            }
        }
    }

    println!("Labels: {:?}", labels);
    Ok(TokenizeResult(labels))

    /*         //split lines with eol
    let lines = code.lines();
    for (idx, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        println!("line:{}", line.trim());
        match Instructions::resolve_string(line.trim()) {
            Some(e) => {
                for i in &e {
                    opcode_string.push_str(&format!("{:02X} ", i));
                }
            }
            None => panic!("Invalid instruction: {}", line),
        }
    }
    println!("Opcode string: \"{:?}\"", opcode_string.trim());
    self.get_from_str(&opcode_string.trim()) */
}

pub fn byte_code_parse(compilation_result: TokenizeResult) -> Result<Vec<u8>, AssemblerError> {
    let mut active_memory = 0;
    let mut origin_lines: BTreeMap<u16, String> = BTreeMap::new();
    let mut variables: HashMap<String, u16> = HashMap::new();
    for label in compilation_result.0 {
        active_memory = {
            let origin_value = &label.directives.iter().find(|d| d.0 == "org");
            println!("Origin value: {:?}", origin_value);
            match origin_value {
                Some(origin_value) => match RawAddrCode::parse_assembly_string(&origin_value.1) {
                    Some(e) => match e {
                        RawAddrCode::Absolute(a) => a,
                        _ => {
                            return Err(AssemblerError::InvalidValue("org"));
                        }
                    },
                    None => {
                        return Err(AssemblerError::InvalidValue("org"));
                    }
                },
                None => active_memory,
            }
        };

        variables.insert(label.title.clone(), active_memory);

        for line in label.instructions {
            origin_lines.insert(active_memory, line.clone());
            active_memory += 1;
            println!("Line: {}, active_memory: {}", line, active_memory);
        }
    }

    let mut opc_lines: HashMap<u16, Vec<u8>> = HashMap::new();

    for (k, v) in origin_lines.iter() {
        let mut instruction = v.clone();

        for variable in variables.iter() {
            let value = variable.1.to_string();
            let key = variable.0.to_string();
            instruction = instruction.replace(&key, &value);
        }


        println!("Instruction: -{}-", instruction);
        let opc = Instructions::resolve_string(&instruction);
        opc_lines.insert(*k, opc.unwrap());
    }

    println!("origin_lines lines: {:?}", origin_lines);

    panic!()
}
