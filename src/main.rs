#[derive(paw_structopt::StructOpt, structopt::StructOpt)]
struct Args {
    /// Message to print
    #[structopt(long, default_value = "hello, world!")]
    message: String,
}

#[paw::main]
fn main(args: Args) {
    env_logger::init();
    log::info!("{}", args.message);
}
