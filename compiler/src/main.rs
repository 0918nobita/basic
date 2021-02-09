extern crate compiler;

use clap::{app_from_crate, Arg};
use compiler::{compile, get_io_info, term_color::red_bold, IOInfo, Target};
use std::{
    fs,
    process::{self, Command},
};

struct Options {
    verbose: bool,
    input: String,
}

impl Options {
    fn parse() -> Self {
        let matches = app_from_crate!()
            .arg(Arg::new("INPUT").about("Source file").required(true))
            .arg(
                Arg::new("verbose")
                    .long("verbose")
                    .about("Output verbose messages on internal operations"),
            )
            .get_matches();

        let input = matches.value_of("INPUT").unwrap();
        let verbose = matches.is_present("verbose");

        Options {
            verbose,
            input: input.to_owned(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Options = Options::parse();

    let target = get_target().unwrap_or_default();
    if opts.verbose {
        println!("Target: {:?}", target)
    }

    let IOInfo {
        input: input_info,
        output: output_info,
    } = get_io_info(opts.input)?;

    let content = fs::read_to_string(input_info.src_path)
        .unwrap_or_else(|_| exit_failure("Failed to read the source file"));

    let asm_output = compile(&content).unwrap_or_else(|msg| {
        eprintln!("{}", msg);
        process::exit(1);
    });

    fs::write(&output_info.asm_path, asm_output)?;

    let mut nasm_cmd = Command::new("nasm");
    nasm_cmd.args(&["-f", "elf64", output_info.asm_path.to_str().unwrap()]);

    let status = nasm_cmd.status().unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!(
            "{}",
            red_bold("Unable to find `nasm`, perhaps install NASM and set PATH")
        );
        process::exit(1);
    });

    if !status.success() {
        exit_failure("Error occurs while executing `nasm`");
    }

    let mut ld_cmd = Command::new("ld");
    ld_cmd.args(&[
        "-o",
        output_info.bin_path.to_str().unwrap(),
        output_info.obj_path.to_str().unwrap(),
    ]);

    let status = ld_cmd.status().unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!(
            "{}",
            red_bold("Unable to find `ld`, perhaps install Linker and set PATH")
        );
        process::exit(1);
    });

    if !status.success() {
        exit_failure("Error occurs while executing `ld`");
    }

    Ok(())
}

fn get_target() -> Option<Target> {
    if cfg!(all(target_arch = "x86_64", target_os = "linux")) {
        Some(Target::LinuxX64)
    } else if cfg!(all(target_arch = "x86_64", target_os = "macos")) {
        Some(Target::MacX64)
    } else {
        None
    }
}

fn exit_failure(msg: &str) -> ! {
    eprintln!("{}", red_bold(msg));
    process::exit(1);
}
