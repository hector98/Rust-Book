use std:: {
    env,
    fs,
    process,
    error::Error,
    time::Instant,
};
use minigrep::{search, search_case_insensitive};

fn main() {
    //--> Lectura de los valores del argumento <-- 
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let now = Instant::now();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    let elapsed_time = now.elapsed();
    println!("Running Config::build() took {} seconds.", elapsed_time.as_secs());

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    
    Ok(())
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path,
            ignore_case,
        })
    }
}
