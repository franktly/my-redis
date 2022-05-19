use std::path::PathBuf;
use structopt::StructOpt;
use clap::{App,Arg};

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An axample of structopt")]
struct Opt{
    #[structopt(name = "debug", short, long)]
    debug: bool,

    #[structopt(short = "vec", long = "velocity", default_value = "42" )]
    speed: f64,

    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    #[structopt(short, help = "return the output type of the clapp")]
    out_type: String,

    #[structopt(name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,

    // This option is skipped and will be filled with the 
    // default value for its type(in this case 0)
    #[structopt(skip)]
    skipped: u32,

    // The value(parameter_value) is taken from the specified environment variable(PARAMETER_VALUE)
    // if not given through the command-line
    #[structopt(short, long, env = "PARAMETER_VALUE")]
    parameter_value: String,

    #[structopt(long = "secret", env = "SECRET_VALUE", hide_env_values = true)]
    // Auto deriving enviroment variables when env var is just
    // CASE is different but the same name 
    // equal to `#[structopt(long = "secret", env]`
    secret_value: String,

    // these fileds are to be assigned with Default::default()
    #[structopt(skip)]
    k1: String,
    #[structopt(skip)]
    v1: Vec<u32>,

    // these fileds get set explicitly
    #[structopt(skip = "cake")]
    k2:String,
    #[structopt(skip =vec![1,2,3])]
    v2:Vec<u32>,
}

fn main(){
    let opt = Opt::from_args();
    // println!("{:?}", opt);

    let matches = App::new("AppOpt")
        .help("example help ellipsis")
        .version("0.2.0")
        .author("franktly")
        .about("App Option")
        .arg(Arg::with_name("debug2")
            .help("Active debug2 mode")
            .short("de2")
            .long("debug2"))
        .arg(Arg::with_name("speed2")
            .help("Set speed")
            .short("vec2")
            .long("velocity2")
            .default_value("42"))
        .arg(Arg::with_name("verbo3")
            .short("ver3")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    let dbg = matches.value_of("debug2").unwrap();
    // println!("debug is "{:?}", dbg);

    match matches.occurrences_of("verbo3"){
    0 => println!("No verbose info"),
    1 => println!("Some verbose info"),
    2 => println!("Tons of verbose info"),
    _ => println!("Don't be crazy"),
    }
}
