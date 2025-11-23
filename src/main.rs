use std::env;
use std::fs;
mod vm;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <decicode_file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let code = fs::read_to_string(filename)?;
    let code = code.trim();
    let mut vm = vm::VM::new(code);
    vm.run();

    Ok(())
}
