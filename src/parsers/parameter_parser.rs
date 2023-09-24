use std::error::Error;
use std::path::Path;
use getopts::Options;

#[derive(Debug)]
pub enum ParamError {
    NoDirectory,
    DirectoryDoesNotExist,
    NoOutputDirectory,
    OutputDirectoryDoesNotExist,
    NoGranularity,
    GranularityMustBeANumber,
    ParamInitFailed,
    NoQueue,
    NoQueueURL,
}

fn create_parameters() -> Result<Options, Box<dyn Error>> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("i", "input", "set the input directory", "INPUT");
    opts.optopt("o", "output", "set the output directory", "OUTPUT");
    opts.optopt("g", "granularity", "each chunk's size (in lines)", "GRANULARITY");
    opts.optopt("q", "queue_name", "post chunks to this AMQP queue (requires -u)", "QUEUE_NAME");
    opts.optopt("u", "amqp_url", "set the AMQP queue url", "amqp://user:password@host:port/vhost");
    Ok(opts)
}

fn print_help(opts: Options) {
    let brief = "Usage: csv-splitter [options]";
    print!("{}", opts.usage(&brief));
}

fn parse_parameters() -> Result<[String; 5], ParamError> {

    let Ok(opts) = create_parameters() else {
        return Err(ParamError::ParamInitFailed);    
    };

    let args: Vec<String> = std::env::args().collect();

    let Ok(matches) = opts.parse(&args[1..]) else { return Err(ParamError::ParamInitFailed); };

    if matches.opt_present("h") {
        print_help(opts);
    }

    let Some(dir) = matches.opt_str("i") else
        { return Err(ParamError::NoDirectory); };

    if !Path::new(&dir).is_dir()
        { return Err(ParamError::DirectoryDoesNotExist); }

    let Some(output) = matches.opt_str("o") else
        { return Err(ParamError::NoOutputDirectory); };

    let queue_name: String;
    let amqp_url: String;

    if matches.opt_present("q") {
        if !matches.opt_present("u")
            { return Err(ParamError::NoQueueURL); }

        queue_name = match matches.opt_str("q") {
            Some(q) => q,
            None => return Err(ParamError::NoQueue),
        };

        amqp_url = match matches.opt_str("u") {
            Some(u) => u,
            None => return Err(ParamError::NoQueueURL),
        };
    } else {
        queue_name = String::from("");
        amqp_url = String::from("");
    }

    if !Path::new(&output).is_dir()
        { return Err(ParamError::OutputDirectoryDoesNotExist); }

    let Some(granularity) = matches.opt_str("g") else
        { return Err(ParamError::NoGranularity); };

    if granularity.parse::<u32>().is_err()
        { return Err(ParamError::GranularityMustBeANumber); }

    Ok([dir, output, granularity, queue_name, amqp_url])
}

pub fn get_parameters() -> [String; 5] {
    let params_result = parse_parameters();

    match params_result {
        Ok(params) => return params,
        Err(e) => {
            match e {
                ParamError::NoDirectory => {
                    println!("No input directory specified");
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
                ParamError::NoQueue => {
                    println!("Queue not enabled");
                    std::process::exit(1);
                },
                ParamError::NoQueueURL => {
                    println!("No queue url specified");
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
