use std::fs::{File,remove_file};
use std::io::{prelude::*,BufReader,Result,LineWriter};
use std::env::args;
use std::process::Command;

fn main() -> Result<()> {
    let arguments: Vec<String> = args().collect();
    let source_file = BufReader::new(File::open(&arguments[1])?);
    let mut output_file = LineWriter::new(File::create(&arguments[2])?);
    let mut current_line: u128 = 1;

    //Initilize the file then interpret the source file into valid Rust code.
    output_file.write_all("use std::io::stdin;\n".as_bytes())?;
    output_file.write_all("fn main() {\n".as_bytes())?;
    for line in source_file.lines() {
        proccess(line,&mut output_file,&current_line)?;
        current_line+=1;
    }
    output_file.write("}".as_bytes())?;
    output_file.flush()?;

    //Compile the outputed Rust file.
    println!("Currently compiling file = {}",arguments[2]);
    Command::new("rustc")
            .arg(&arguments[2])
            .status()
            .expect("Failed to compile output file, may need to be manually compiled.");
    remove_file(&arguments[2])?;
    Ok(())
}

//Proccess a line and writes data to the output file.
fn proccess(line: Result<String>,file: &mut LineWriter<File>, current_line: &u128) -> Result<()> {
    //Proccess the line and then write the interpreted output to file.
    let unwraped_line = line.unwrap();
    let splt: Vec<&str> = unwraped_line.split(':').collect();
    
    //Do advanced for special keywords proccessing.
    match splt[0] {
        "const" => {
            file.write_all("let ".as_bytes())?;
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" = ".as_bytes())?;
            file.write_all(splt[2].as_bytes())?;
            file.write_all(";\n".as_bytes())?;
        },
        "var" => {
            file.write_all("let mut ".as_bytes())?;
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" = ".as_bytes())?;
            file.write_all(splt[2].as_bytes())?;
            file.write_all(";\n".as_bytes())?;
        },
        "print" => {
            file.write_all("println!(\"{}\",".as_bytes())?;
            file.write_all(splt[1].as_bytes())?;
            file.write_all(");\n".as_bytes())?;
        },
        "add" => {
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" += ".as_bytes())?;
            file.write_all(splt[2].as_bytes())?;
            file.write_all(";\n".as_bytes())?;
        },
        "sub" => {
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" -= ".as_bytes())?;
            file.write_all(splt[2].as_bytes())?;
            file.write_all(";\n".as_bytes())?;
        },
        "mult" => {
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" *= ".as_bytes())?;
            file.write_all(splt[2].as_bytes())?;
            file.write_all(";\n".as_bytes())?;
        },
        "div" => {
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" /= ".as_bytes())?;
            file.write_all(splt[2].as_bytes())?;
            file.write_all(";\n".as_bytes())?;
        },
        "if" => {
            file.write_all("if ".as_bytes())?;
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" {\n".as_bytes())?;
        },
        "end" => {
            file.write_all("}\n".as_bytes())?;
        },
        "ifnz" => {
            file.write_all("if ".as_bytes())?;
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" != 0 {\n".as_bytes())?;
        },
        "while" => {
            file.write_all("while ".as_bytes())?;
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" {\n".as_bytes())?;
        },
        "whilenz" => {
            file.write_all("while ".as_bytes())?;
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" != 0 {\n".as_bytes())?;
        },
        "finish" => {
            file.write_all("return;\n".as_bytes())?;
        },
        "set" => {
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" = ".as_bytes())?;
            file.write_all(splt[2].as_bytes())?;
            file.write_all(";\n".as_bytes())?;
        },
        "input" => {
            file.write_all(splt[1].as_bytes())?;
            file.write_all(" = String::new();\n".as_bytes())?;
            file.write_all("stdin()".as_bytes())?;
            file.write_all(".read_line(&mut ".as_bytes())?;
            file.write_all(splt[1].as_bytes())?;
            file.write_all(")\n".as_bytes())?;
            file.write_all(".expect(\"Failed to read line\");\n".as_bytes())?;
        },
        "parse" => {
            file.write_all(splt[2].as_bytes())?;
            file.write_all(" = ".as_bytes())?;
            file.write_all(splt[1].as_bytes())?;
            file.write_all(".trim().parse().unwrap();\n".as_bytes())?;  
        },
        _ => {println!("COMMAND NOT FOUND {{ {} }} COMMAND IGNORED AT LINE: {}",splt[0],current_line);}
    }
    Ok(())
}