use clap::Parser;

mod literals;

/// The ToyQL query engine.
///
/// Processes queries, either from the command line or from a file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Files containing query text
    #[arg(short = 'f', long = "file")]
    query_files: Vec<String>,

    /// Literal query text
    queries: Vec<String>,
}

fn run_from_args(args: Args) {
    for file in args.query_files {
        println!("I would parse the external file {}", file);
    }
    for query in args.queries {
        println!("Executing the query {}", query);
        let parsed_literal = literals::parsing::literal(&query);
        match parsed_literal {
            Ok((r, literals::LiteralValue::Int(i))) => println!("Got int {} ...{}", i, r),
            Ok((r, literals::LiteralValue::Float(f))) => println!("Got float {} ...{}", f, r),
            Ok((r, literals::LiteralValue::String(s))) => println!("Got string \"{}\" ...{}", s, r),
            Err(e) => panic!("parse error {}", e),
        }
    }
}

/// The "real" main entry point of our binary target, relocated to a lib so that
/// rust-test can reach it.
pub fn _main() {
    let args = Args::parse();

    run_from_args(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        run_from_args(Args {
            query_files: vec![],
            queries: vec![],
        });
    }
}
