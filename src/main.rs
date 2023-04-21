use std::path::Path;

use clap::Parser;
use env_logger::Env;
use lib::{Lexer, Scanner};
use log::{debug, error, info, trace, warn};

#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    startup_logger();
    let args = Args::parse();
    let file = args.file;
    let scanner = Scanner::from_path(Path::new(&file)).expect("where file");
    let lexer = Lexer::new(scanner);
    lexer.for_each(|t| println!("{:?}", t));
    info!("Finished Compiling Succesfully");
}

const DEBUG_LOG: &'static str = "debug";
const RELEASE_LOG: &'static str = "info";

fn startup_logger() {
    let (default_filter, default_write) = match in_release_build() {
        true => (RELEASE_LOG, "always"),
        false => (DEBUG_LOG, "always"),
    };

    let env = Env::default()
        .filter_or("MY_LOG", default_filter)
        .write_style_or("MY_LOG_STYLE", default_write);

    env_logger::init_from_env(env);

    trace!("This is a trace log. If you are running a release build, this should NOT be visible.");
    debug!("This is a debug log. This should only be visible in debug builds.");
    info!("This is an info log. If you are in a release build, you should see [INFO] [WARN] and [ERROR] logs only.");
    warn!("This is a warning log. This should be visible in all builds.");
    error!("This is an error log. If you see this, something has gone horribly wrong.");

    info!("");
    info!("");
    info!("Logger set up successfully!");
}

#[cfg(debug_assertions)]
const fn in_release_build() -> bool {
    false
}

#[cfg(not(debug_assertions))]
#[inline(always)]
const fn in_release_build() -> bool {
    true
}
