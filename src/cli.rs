use clap::{Arg, Command, builder::FalseyValueParser};
use log::{debug, error};
use std::{env, path::Path, process};

pub fn start() {
    let cli_matches = Command::new("Pascal compiler")
        .author("Wes Chappell")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Pascal compiler CLI")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("filepath to code entrypoint")
                .num_args(1)
                .value_name("FILE")
                .required(true),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Starts an interactive pascal interpreter")
                .num_args(0..=1)
                .value_parser(FalseyValueParser::new()),
        )
        .after_help("Pascal compiler & interpreter CLI")
        .try_get_matches()
        .unwrap_or_else(|e| {
            error!("Failed to parse CLI args {}", e.to_string());
            e.exit()
        });

    if cli_matches.args_present() {
        debug!("Args are present, parsing...");
        cli_matches.get_one::<String>("file").map(|s| {
            let buf = Path::new(s);
            let curr_dir = env::current_dir().unwrap();
            let abs_path = curr_dir.join(buf);
            let canon_abs_path = abs_path.canonicalize().unwrap_or_else(|e| {
                error!(
                    "Failed to cannonicalize the path {} {}",
                    abs_path.display(),
                    e.to_string()
                );
                process::exit(1);
            });

            if !canon_abs_path.is_file() {
                error!("{} is not a file.", canon_abs_path.display());
                process::exit(1);
            }

            if let Some(file_ext) = canon_abs_path.extension() {
                if let Some(file_ext_str) = file_ext.to_str() {
                    if file_ext_str != "pas" {
                        error!("{} is not a pascal file.", canon_abs_path.display());
                        process::exit(1);
                    }
                }
            }

            return abs_path;
        });
    }
}
