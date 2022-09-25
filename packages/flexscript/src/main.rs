use args::Args;
use clap::Parser;
use flexscript_parser::parse_raw_ast;

mod args;

fn main() {
    let args = Args::parse();

    match args.command {
        args::Commands::Rawast(a) => {
            println!("Rawast {}", a.path);

            let text = std::fs::read_to_string(a.path).unwrap();

            let res = parse_raw_ast(&text).unwrap();

            println!("{:#?}", res);
        }
    }
}
