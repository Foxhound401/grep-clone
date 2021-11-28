use structopt::StructOpt;

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[structopt(parse(from_os_str))]
    #[structopt(short = "o", long = "output")]
    path: std::path::PathBuf,
}

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };


    let args = Cli::from_args();

    // println!("{:?}", args)
    let content = std::fs::read_to_string(&args.path)
        .expect("Could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };

    println!("File content: {}", content);
}
