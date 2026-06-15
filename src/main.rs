mod config;
mod llm;
mod msg;
use clap::Parser;
use msg::Msg;
use std::vec;

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
        let inputMsgs: vec::Vec<Msg> = vec![
            Msg {
                role: String::from("system"),
                content: String::from(
                    "Give a precise answer, no need to explain. if user asks for a command just give the command as response or if user asks for something else just give direct answer in one or two lines.",
                ),
            },
            Msg {
                role: String::from("user"),
                content: args.query,
            },
        ];
        print!(" using url {} and model {}", config.base_url, config.model);
        llm::query_llm(
            format!("{}/api/chat", config.base_url),
            config.model,
            inputMsgs,
        );
    }
}
