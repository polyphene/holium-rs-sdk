use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};

mod build;
mod info;
mod utils;

fn main() {
    // Create CLI matches
    let matches = App::new("Holium")
        .bin_name("holium-sdk-cli")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Develop module for the Holium protocol!")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("build")
                .about("Build a valid Holium module from a Rust project")
                .setting(AppSettings::TrailingVarArg)
                .setting(AppSettings::AllowLeadingHyphen)
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT_DIR")
                        .help("Sets a custom output directory"),
                )
                .arg(Arg::from_usage("[OPTIONAL]... 'Cargo build arguments'").multiple(true)),
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("Get information on a Holium module")
                .setting(AppSettings::TrailingVarArg)
                .setting(AppSettings::AllowLeadingHyphen)
                .arg(
                    Arg::with_name("INPUT") // And their own arguments
                        .help("Holium module bytecode")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    // Match subcommands
    let exec_res = match matches.subcommand() {
        ("build", Some(build_matches)) => crate::build::build(build_matches),
        ("info", Some(info_matches)) => crate::info::info(info_matches),
        _ => unreachable!(),
    };

    // Use execution result
    std::process::exit(match exec_res {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    })
}
