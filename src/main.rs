#![allow(dead_code)]
#![allow(unused_mut)]
#[allow(unused_imports)]
use std::collections::HashMap;
use std::env;
use std::panic;

mod lang;
use lang::*;

fn main() -> Result<(), ()> {
    // /*
    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));
    // */
    //env_logger::init();
    /*
        error: The following required arguments were not provided:
        <path>

    USAGE:
        lang <path>
    */
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "error: The following required arguments were not provided:
        <path>

    USAGE:
        lang <path>

    For more information try --help"
        );
        panic!();
    }

    if &args[1] == "--help" {
        println!(
            "USAGE:
        lang <path>"
        );
        return Ok(());
    }

    let path = &args[1];
    let split: Vec<&str> = path.as_str().split('.').collect();
    if !(split[split.len() - 1] == "lang") {
        println!(
            "File {:?} has a unexpected file extension of .{}",
            path,
            split[split.len() - 1]
        );
        panic!();
    }
    let mut content: String;
    match std::fs::read_to_string(path) {
        Ok(x) => content = x,
        Err(_e) => {
            println!("could not read file `{}`", path);
            panic!();
        }
    }

    let mut con: Vec<String> = content.clone().split('\n').map(|x| x.to_string()).collect();

    let mut tok = tokenizer::Tokenizer::new(content, con.clone());
    let (token_stream, pos) = tok.make_tokens(); // create preprocesser to check that all ( and { have matches and for () to make bool or int

    //println!("stream: {:#?}", token_stream);
    let mut prepro = preprocesser::Preprocesser::new(token_stream, &pos, con);
    let processed_stream = prepro.process();
        
    let mut lex = lexer::Lexer::new(processed_stream);
    let tree = lex.tree();
    //println!("tr {:?}", tree);
    let special_forms = specialforms::Specialforms::new();

    let mut eval = evaluate::Evaluate {
        special_forms: special_forms,
    };

    let mut scope = scope::Scope::new();

    eval.evaluate(tree, &mut scope); // for each value check if needed +-/* or bool
    Ok(())
}

//x86_64-pc-windows-gnu
//x86_64-apple-darwin
//aarch64-unknown-linux-gnu

//cargo build --release --target {}

//top, lk {pid}
