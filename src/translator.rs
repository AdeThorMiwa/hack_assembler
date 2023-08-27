pub enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

pub struct CommandTranslator {
    #[allow(dead_code)]
    command: String,

    pub command_type: CommandType,
}

impl CommandTranslator {
    pub fn new(command: &str) -> Self {
        let command_type = {
            if command.find("@").is_some() {
                CommandType::ACommand
            } else if command.find("(").is_some() && command.find(")").is_some() {
                CommandType::LCommand
            } else {
                CommandType::CCommand
            }
        };

        Self {
            command: command.to_string(),
            command_type,
        }
    }

    pub fn get_symbol_value(&self) -> String {
        self.command[1..].to_string()
    }

    pub fn get_comp_value(&self) -> Option<String> {
        if let Some(index) = self.command.find("=") {
            let (_, comp) = self.command.split_at(index + 1);
            // println!("cmpvalue={}", comp);
            return Some(comp.to_owned());
        }

        if let Some(index) = self.command.find(";") {
            let (comp, _) = self.command.split_at(index + 1);
            // println!("cmpvalue={}", comp);
            return Some(comp.replace(";", ""));
        }

        None
    }

    pub fn get_dest_value(&self) -> Option<String> {
        if let Some(index) = self.command.find("=") {
            let (dest, _) = self.command.split_at(index);
            // println!("destvalue={}", dest);
            return Some(dest.to_owned());
        }

        None
    }

    pub fn get_jmp_value(&self) -> Option<String> {
        if let Some(index) = self.command.find(";") {
            let (_, jump) = self.command.split_at(index + 1);
            // println!("jumpvalue={}", jump);
            return Some(jump.to_owned());
        }

        None
    }

    pub fn is_memory_op(&self) -> bool {
        if let Some(comp) = self.get_comp_value() {
            if comp.contains("M") {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::CommandTranslator;

    #[test]
    fn test_get_symbol_value() {
        assert_eq!(CommandTranslator::new("@100").get_symbol_value(), "100");
        assert_eq!(CommandTranslator::new("@sum").get_symbol_value(), "sum")
    }
}
