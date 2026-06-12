mod config;

use clap::Parser;

#[derive(Parser)]
#[command(name = "qhelp")]
struct Cli {
    query: String,
    #[arg(short)]
    e: bool,
}

fn main() {
    // println!("hey ! i am quick help.");
    let args = Cli::parse();
    let config = match config::load_config() {
        Ok(config) => config,
        Err(e) => {
            eprint!("{}", e);
            std::process::exit(1);
        }
    };
    if args.e {
        println!(
            "I will query {} then give detailed explanation with answer.",
            args.query
        );
    } else {
        println!(
            "I will query {}, then give compact and precise answer.",
            args.query
        );
    }
}
