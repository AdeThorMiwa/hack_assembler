use std::collections::HashMap;

use crate::{parser::Parser, utils::is_numeric};

#[derive(Debug)]
pub struct SymbolTable(HashMap<String, u32>);

impl SymbolTable {
    pub fn new(contents: String) -> Self {
        let mut parser = Parser::new(contents);
        let table = Self::build(&mut parser);
        Self(table)
    }

    pub fn get(&self, key: &str) -> Option<u32> {
        self.0.get(key).map(|i| *i)
    }
}

impl SymbolTable {
    fn build(parser: &mut Parser) -> HashMap<String, u32> {
        let mut table = HashMap::new();

        // load predefined symbols
        Self::load_predefined_symbols(&mut table);

        // load label symbols
        Self::load_label_symbols(parser, &mut table);

        // load variable symbols
        Self::load_variable_symbols(parser, &mut table);

        table
    }

    fn load_predefined_symbols(table: &mut HashMap<String, u32>) {
        // R(n) symbols
        table.insert("R0".to_string(), 0);
        table.insert("R1".to_string(), 1);
        table.insert("R2".to_string(), 2);
        table.insert("R3".to_string(), 3);
        table.insert("R4".to_string(), 4);
        table.insert("R5".to_string(), 5);
        table.insert("R6".to_string(), 6);
        table.insert("R7".to_string(), 7);
        table.insert("R8".to_string(), 8);
        table.insert("R9".to_string(), 9);
        table.insert("R10".to_string(), 10);
        table.insert("R11".to_string(), 11);
        table.insert("R12".to_string(), 12);
        table.insert("R13".to_string(), 13);
        table.insert("R14".to_string(), 14);
        table.insert("R15".to_string(), 15);

        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);
        table.insert("SP".to_string(), 0);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);
    }

    fn load_label_symbols(parser: &mut Parser, table: &mut HashMap<String, u32>) {
        let mut line_number = 0;
        while let Some(line) = parser.next() {
            if let Some(start) = line.find("(") {
                if let Some(end) = line.find(")") {
                    let label = line[(start + 1)..end].to_string();
                    table.insert(label, line_number);
                    continue;
                }
            }
            line_number += 1;
        }
    }

    fn load_variable_symbols(parser: &mut Parser, table: &mut HashMap<String, u32>) {
        let mut next_memory = 16;
        parser.reset();
        while let Some(line) = parser.next() {
            if let Some(index) = line.find("@") {
                let (_, variable) = line.split_at(index + 1);
                if !table.contains_key(variable) && !is_numeric(variable) {
                    table.insert(variable.to_string(), next_memory);
                    next_memory += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::Parser;

    use super::SymbolTable;

    #[test]
    fn test_load_label_symbols() {
        let mut parser = Parser::new(
            r#"@i
M=1
@sum
M=0
(LOOP)
@i
D=M
@100
D=D-A
@END
D;JGT
@i
D=M
@sum
M=D+M
@i
M=M+1
@LOOP
0;JMP
(END)
@END
0;JMP
"#
            .to_string(),
        );

        let mut table = HashMap::new();
        let _ = SymbolTable::load_label_symbols(&mut parser, &mut table);

        assert_eq!(table.get("LOOP"), Some(&4));
        assert_eq!(table.get("END"), Some(&18))
    }

    #[test]
    fn test_load_variable_symbols() {
        let mut parser = Parser::new(
            r#"@i
M=1
@sum
M=0
(LOOP)
@i
D=M
@100
D=D-A
@END
D;JGT
@i
D=M
@sum
M=D+M
@i
M=M+1
@LOOP
0;JMP
(END)
@END
0;JMP
"#
            .to_string(),
        );

        let mut table = HashMap::new();
        let _ = SymbolTable::load_variable_symbols(&mut parser, &mut table);

        assert_eq!(table.get("i"), Some(&16));
        assert_eq!(table.get("sum"), Some(&17))
    }
}
