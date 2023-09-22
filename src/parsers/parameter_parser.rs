use std::error::Error;
use std::path::Path;
use getopts::Options;
use dotenv::dotenv;
use dotenv::var;

#[derive(Debug)]
pub enum ParamError {
    NoDirectory,
    DirectoryDoesNotExist,
    NoOutputDirectory,
    OutputDirectoryDoesNotExist,
    NoGranularity,
    GranularityMustBeANumber,
    ParamInitFailed,
}

fn create_parameters() -> Result<Options, Box<dyn Error>> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the version");
    opts.optopt("d", "directory", "set the input directory", "DIRECTORY");
    opts.optopt("o", "output", "set the output directory", "OUTPUT");
    opts.optopt("g", "granularity", "each chunk's size (in lines)", "GRANULARITY");
    Ok(opts)
}

fn print_help(opts: Options) {
    let brief = "Usage: csv-splitter [options]";
    print!("{}", opts.usage(&brief));
}

fn parse_parameters() -> Result<[String; 3], ParamError> {
    dotenv().ok();

    let Ok(opts) = create_parameters() else {
        return Err(ParamError::ParamInitFailed);    
    };

    let args: Vec<String> = std::env::args().collect();

    let Ok(matches) = opts.parse(&args[1..]) else {
        print_help(opts);
        std::process::exit(0);
    };
    if matches.opt_present("h") {
        print_help(opts);
        std::process::exit(0);
    }
    if matches.opt_present("v") {
        let version = var("CSV_SPLITTER_VERSION").unwrap();
        println!("csv-splitter v{}", version);
        std::process::exit(0);
    }

    let Some(dir) = matches.opt_str("d") else
        { return Err(ParamError::NoDirectory); };

    if !Path::new(&dir).is_dir()
        { return Err(ParamError::DirectoryDoesNotExist); }

    let Some(output) = matches.opt_str("o") else
        { return Err(ParamError::NoOutputDirectory); };

    if !Path::new(&output).is_dir()
        { return Err(ParamError::OutputDirectoryDoesNotExist); }

    let Some(granularity) = matches.opt_str("g") else
        { return Err(ParamError::NoGranularity); };

    if granularity.parse::<u32>().is_err()
        { return Err(ParamError::GranularityMustBeANumber); }

    Ok([dir, output, granularity])
}

pub fn get_parameters() -> [String; 3] {
    let params_result = parse_parameters();

    match params_result {
        Ok(params) => return params,
        Err(e) => {
            match e {
                ParamError::NoDirectory => {
                    println!("No directory specified");
                    std::process::exit(1);
                },
                ParamError::DirectoryDoesNotExist => {
                    println!("Directory does not exist");
                    std::process::exit(1);
                },
                ParamError::NoOutputDirectory => {
                    println!("No output directory specified");
                    std::process::exit(1);
                },
                ParamError::OutputDirectoryDoesNotExist => {
                    println!("Output directory does not exist");
                    std::process::exit(1);
                },
                ParamError::NoGranularity => {
                    println!("No granularity specified");
                    std::process::exit(1);
                },
                ParamError::GranularityMustBeANumber => {
                    println!("Granularity must be a number");
                    std::process::exit(1);
                },
                ParamError::ParamInitFailed => {
                    println!("Failed to initialize parameters");
                    std::process::exit(1);
                },
            }
        }
    }
}
