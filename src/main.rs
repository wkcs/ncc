mod cmdline;
mod lex;

use std::path::Path;

use cmdline as cmd;
use lex::Lex;

fn add_cmd_info(cmdline: &mut cmd::CmdLine) {
    cmdline.add(
        "",
        "--help",
        "Show this help info.",
        cmd::CmdValType::NoVal,
        "",
    );
    cmdline.add(
        "",
        "--version",
        "Show version number.",
        cmd::CmdValType::NoVal,
        "",
    );
    cmdline.add(
        "-o",
        "",
        "Place the output into <file>.",
        cmd::CmdValType::ValSpace,
        "file",
    );
    cmdline.add(
        "-D",
        "",
        "Add macro definition.",
        cmd::CmdValType::ValOptSpace,
        "macro[=<value>]",
    );
    cmdline.add(
        "-I",
        "",
        "Add the header file index path.",
        cmd::CmdValType::ValOptSpace,
        "path",
    );
    cmdline.add(
        "-v",
        "",
        "Display the programs invoked by the compiler.",
        cmd::CmdValType::NoVal,
        "",
    );
    cmdline.add(
        "-###",
        "",
        "Like -v but options quoted and commands not executed.",
        cmd::CmdValType::NoVal,
        "",
    );
    cmdline.add(
        "-E",
        "",
        "Preprocess only; do not compile, assemble or link.",
        cmd::CmdValType::NoVal,
        "",
    );
    cmdline.add(
        "-S",
        "",
        "Compile only; do not assemble or link.",
        cmd::CmdValType::NoVal,
        "",
    );
    cmdline.add(
        "-c",
        "",
        "Compile and assemble, but do not link.",
        cmd::CmdValType::NoVal,
        "",
    );
    cmdline.add(
        "-pie",
        "",
        "Create a dynamically linked position independent executable.",
        cmd::CmdValType::NoVal,
        "",
    );
    cmdline.add(
        "-std=",
        "",
        "Set language standards for use.",
        cmd::CmdValType::ValNoSpace,
        "",
    );
}

fn check_input_file(files: &Vec<String>) {
    let mut err = false;

    for file in files {
        if !Path::new(file).exists() {
            eprintln!("{}: No such file", file);
            err = true;
        }
    }

    if err {
        eprintln!("\nError: Input file error\n");
        std::process::exit(-1);
    }
}

fn main() {
    let mut cmdline = cmd::CmdLine::new();
    let args: Vec<String> = std::env::args().collect();
    let test = 1.2;
    let test2: f64 = 3.4f64;
    let test3 = 0b10101;
    let test4 = 0xaBcdef;
    let test5 = 01234567;

    add_cmd_info(&mut cmdline);
    cmdline.parse(&args[1..].to_vec());

    if cmdline.is_include("--help") {
        println!(
            "{}\n\nNcc compiler by Nick.Hu -- {}",
            cmdline.help(),
            "V0.1.0"
        );
        std::process::exit(0);
    }

    if cmdline.is_include("--version") {
        println!("Ncc compiler by Nick.Hu -- {}", "V0.1.0");
        std::process::exit(0);
    }

    if cmdline.others.len() == 0 {
        eprintln!("No input file");
        std::process::exit(-1);
    }
    check_input_file(&cmdline.others);

    /*
    if let Some(vals) = cmdline.get_value_by_name("-D") {
        println!("[{}]:{:?}", "-D", vals);
    }
    if let Some(vals) = cmdline.get_value_by_name("-I") {
        println!("[{}]:{:?}", "-I", vals);
    }

    println!("[others]:{:?}", cmdline.others);
    println!("{:?}", cmdline.args);
    */

    let mut lex = Lex::new(&cmdline.others[0]);
    lex.parse();
    println!("{}", lex.show());
}
