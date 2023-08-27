use anyhow::{anyhow, Context};
use std::io::Write;
use std::{fs::File, io::BufWriter, path::PathBuf};

use crate::lookup_tables::LookUpTable;
use crate::utils::{dec_to_bin, has_valid_extension, is_numeric};
use crate::{
    constants::{ASSEMBLY_FILE_TYPE, HACK_FILE_TYPE},
    parser::Parser,
    symbol_table::SymbolTable,
    translator::CommandTranslator,
};

pub fn assemble(file_name: &str, output_path: Option<String>) -> anyhow::Result<()> {
    let file_path = PathBuf::from(&file_name);

    if !has_valid_extension(&file_path, ASSEMBLY_FILE_TYPE) {
        return Err(anyhow!(
            "the assemble command expect a file with the .{} extension",
            ASSEMBLY_FILE_TYPE
        ))
        .context("Invalid file type");
    }

    // read file
    let file_path = PathBuf::from(file_name);
    let file_contents = std::fs::read_to_string(file_path.clone())
        .with_context(|| format!("could not read file `{}`", file_name))?;

    // parse file - remove comment and white space
    let mut parser = Parser::new(file_contents.clone());

    // init symbol table
    let symbol_table = SymbolTable::new(file_contents.clone());

    // init lookup table
    let code_lookup_table = LookUpTable::new();

    // second pass
    // create a file writer to write to
    let mut out_file = if let Some(path) = output_path {
        PathBuf::from(path)
    } else {
        file_path.clone()
    };
    out_file.set_extension(HACK_FILE_TYPE);
    let output_file_name = out_file.to_str().unwrap();

    let mut writer = BufWriter::new(
        File::create(output_file_name)
            .with_context(|| format!("failed to create file `{}`", output_file_name))?,
    );

    // for each line
    while let Some(cmd) = parser.next() {
        //  if line is A-commands
        let command = CommandTranslator::new(&cmd);

        match command.command_type {
            crate::translator::CommandType::ACommand => {
                println!("A Command >>> {}", cmd);
                let symbol = {
                    let sym = command.get_symbol_value();
                    if is_numeric(&sym) {
                        sym.parse::<u32>().unwrap()
                    } else {
                        symbol_table.get(&sym).unwrap()
                    }
                };

                let bin_value = dec_to_bin(symbol, Some(15));
                writeln!(&mut writer, "{}{}", 0, bin_value)?;
            }
            crate::translator::CommandType::LCommand => {}
            crate::translator::CommandType::CCommand => {
                println!("C Command >>> {}", cmd);
                let g = command.get_dest_value().unwrap_or("NULL".to_string());
                println!("dest={g}");
                let dest = code_lookup_table.dest.get(&g).unwrap();

                let comp = code_lookup_table
                    .comp
                    .get(&command.get_comp_value().unwrap_or("0".to_string()))
                    .unwrap();
                let jump = code_lookup_table
                    .jmp
                    .get(&command.get_jmp_value().unwrap_or("NULL".to_string()))
                    .unwrap();

                let a = if command.is_memory_op() { 1 } else { 0 };
                writeln!(&mut writer, "{}11{}{}{}{}", 1, a, comp, dest, jump)?;
            }
        }
    }

    writer.flush().unwrap();

    Ok(())
}
