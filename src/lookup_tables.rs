use std::collections::HashMap;

pub struct LookUpTable {
    pub dest: HashMap<String, String>,
    pub jmp: HashMap<String, String>,
    pub comp: HashMap<String, String>,
}

impl LookUpTable {
    pub fn new() -> Self {
        let dest = LookUpTable::get_dest_table();
        let jmp = LookUpTable::get_jmp_table();
        let comp = LookUpTable::get_comp_table();
        Self { dest, jmp, comp }
    }

    fn get_dest_table() -> HashMap<String, String> {
        let mut dest_table: HashMap<String, String> = HashMap::new();
        dest_table.insert("NULL".to_string(), "000".to_string());
        dest_table.insert("M".to_string(), "001".to_string());
        dest_table.insert("D".to_string(), "010".to_string());
        dest_table.insert("DM".to_string(), "011".to_string());
        dest_table.insert("MD".to_string(), "011".to_string());
        dest_table.insert("A".to_string(), "100".to_string());
        dest_table.insert("AM".to_string(), "101".to_string());
        dest_table.insert("MA".to_string(), "101".to_string());
        dest_table.insert("AD".to_string(), "110".to_string());
        dest_table.insert("DA".to_string(), "110".to_string());
        dest_table.insert("ADM".to_string(), "111".to_string());
        dest_table.insert("AMD".to_string(), "111".to_string());
        dest_table.insert("DAM".to_string(), "111".to_string());
        dest_table.insert("DMA".to_string(), "111".to_string());
        dest_table.insert("MAD".to_string(), "111".to_string());
        dest_table.insert("MDA".to_string(), "111".to_string());
        dest_table
    }

    fn get_jmp_table() -> HashMap<String, String> {
        let mut jmp_table: HashMap<String, String> = HashMap::new();

        jmp_table.insert("NULL".to_string(), "000".to_string());
        jmp_table.insert("JGT".to_string(), "001".to_string());
        jmp_table.insert("JEQ".to_string(), "010".to_string());
        jmp_table.insert("JGE".to_string(), "011".to_string());
        jmp_table.insert("JLT".to_string(), "100".to_string());
        jmp_table.insert("JNE".to_string(), "101".to_string());
        jmp_table.insert("JLE".to_string(), "110".to_string());
        jmp_table.insert("JMP".to_string(), "111".to_string());
        jmp_table
    }

    fn get_comp_table() -> HashMap<String, String> {
        let mut comp_table: HashMap<String, String> = HashMap::new();

        // if a == 0
        comp_table.insert("0".to_string(), "101010".to_string());
        comp_table.insert("1".to_string(), "111111".to_string());
        comp_table.insert("-1".to_string(), "111010".to_string());
        comp_table.insert("D".to_string(), "001100".to_string());
        comp_table.insert("A".to_string(), "110000".to_string());
        comp_table.insert("!D".to_string(), "001101".to_string());
        comp_table.insert("!A".to_string(), "110001".to_string());
        comp_table.insert("-D".to_string(), "001111".to_string());
        comp_table.insert("-A".to_string(), "110011".to_string());
        comp_table.insert("D+1".to_string(), "011111".to_string());
        comp_table.insert("A+1".to_string(), "110111".to_string());
        comp_table.insert("D-1".to_string(), "001110".to_string());
        comp_table.insert("A-1".to_string(), "110010".to_string());
        comp_table.insert("D+A".to_string(), "000010".to_string());
        comp_table.insert("D-A".to_string(), "010011".to_string());
        comp_table.insert("A-D".to_string(), "000111".to_string());
        comp_table.insert("D&A".to_string(), "000000".to_string());
        comp_table.insert("D|A".to_string(), "010101".to_string());

        // if a == 1
        comp_table.insert("M".to_string(), "110000".to_string());
        comp_table.insert("!M".to_string(), "110001".to_string());
        comp_table.insert("-M".to_string(), "110011".to_string());
        comp_table.insert("M+1".to_string(), "110111".to_string());
        comp_table.insert("M-1".to_string(), "110010".to_string());
        comp_table.insert("D+M".to_string(), "000010".to_string());
        comp_table.insert("D-M".to_string(), "010011".to_string());
        comp_table.insert("M-D".to_string(), "000111".to_string());
        comp_table.insert("D&M".to_string(), "000000".to_string());
        comp_table.insert("D|M".to_string(), "010101".to_string());

        comp_table
    }
}
