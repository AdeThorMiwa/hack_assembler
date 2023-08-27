use crate::symbol_table::SymbolTable;

#[derive(Debug, Clone)]
pub struct Parser {
    contents: Vec<String>,
    current_line: u32,
    total_lines: u32,
}

impl Parser {
    pub fn new(contents: String) -> Self {
        let contents = Self::strip_comments(&contents);
        let contents = Self::strip_whitespaces(&contents);

        let contents = contents.lines().map(String::from).collect::<Vec<String>>();

        let total_lines = contents.len() as u32;

        Self {
            contents,
            current_line: 0,
            total_lines,
        }
    }

    pub fn reset(&mut self) {
        self.current_line = 0
    }

    fn strip_comments(contents: &str) -> String {
        // for each line
        // traverse till end
        // if any // and is first char on line, remove entire line
        // if any // and not first chars, slice till line break
        let mut stripped = String::new();
        for line in contents.lines() {
            let start_of_comment = line.find("//");
            if let Some(index) = start_of_comment {
                if index == 0 {
                    continue;
                }

                let (command, _) = line.split_at(index);
                stripped.push_str(&format!("{}\n", command));
            } else {
                stripped.push_str(&format!("{}\n", line))
            }
        }

        stripped
    }

    fn strip_whitespaces(contents: &str) -> String {
        // for each line
        // filter whitespace
        let mut stripped = String::new();

        for line in contents.lines() {
            let stripped_line = line
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();
            if stripped_line.trim().len() > 0 {
                stripped.push_str(&format!("{}\n", stripped_line));
            }
        }

        stripped
    }

    pub fn update_symbols(&mut self, _symbol_table: SymbolTable) {}
}

impl Iterator for Parser {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_line >= self.total_lines {
            return None;
        }

        let line_number = self.current_line;
        self.current_line += 1;

        let v = self.contents.get(line_number as usize);

        v.map(String::from)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test_strip_comment() {
        let asm = r#"
// Adds 1 + ... + 100
    @i
    M=1 // i=1
    @sum
    M=0 // sum=0
"#;
        let striped = Parser::strip_comments(asm);
        assert_eq!(
            striped,
            r#"
    @i
    M=1 
    @sum
    M=0 
"#
        );
    }

    #[test]
    fn test_strip_whitespace() {
        let asm = r#"
    @i
    M=1 
    @sum
    M=0 
"#;
        let stripped = Parser::strip_whitespaces(asm);
        assert_eq!(
            stripped,
            r#"
@i
M=1
@sum
M=0
"#
        );
    }
}
