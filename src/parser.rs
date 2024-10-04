use std::collections::HashMap;

use crate::asm::{Instructions, RawAddrCode};

#[derive(Debug)]
pub enum AssemblerError {
    InvalidDirectiveLine(usize),
    InvalidDirectiveValue(usize),
    InvalidInstruction(usize),
    InvalidLabel(usize),
}

#[derive(Debug)]
pub struct AssemblerDirective(String, String);

/// Parse the program from given string
pub fn parse_assembly_string(code: &str) -> Result<(), AssemblerError> {
    //split lines with eol
    let lines = code.lines();
    let mut active_scope = "_".to_string();
    let mut label_map: HashMap<String, Vec<String>> = HashMap::new();
    label_map.insert(active_scope.clone(), Vec::new());

    let mut directives = Vec::new();

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
                directives.push(AssemblerDirective(directive.to_string(), value.to_string()));
            } else {
                let mut formated_line = line.trim().to_string();
                if line.contains(";") {
                    formated_line = formated_line.split(";").collect::<Vec<_>>()[0].to_string();
                }

                label_map
                    .get_mut(&active_scope)
                    .unwrap()
                    .push(formated_line);
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
                label_map.insert(active_scope.clone(), Vec::new());
            } else {
                return Err(AssemblerError::InvalidLabel(idx + 1));
            }
        }
    }

    panic!("directives: {:#?}\nLabel map: {:#?}", directives, label_map);

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
