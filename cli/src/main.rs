use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
        test: String,
}

fn main() {
    let args = Cli::from_args();

    let out: String = match args.test.as_str() {
        "test1" => "test1".to_string(),
        _ => "Undefined".to_string(),
    };

    println!("{}", out)
}
