// Author: Lachlan Gunn
//
// Copyright:
//   2018 Secure Systems Group, Aalto University https://ssg.aalto.fi/
//
// This code is released under the Apache 2.0 licence.

use ::clap::{Arg, App, AppSettings, SubCommand};

pub fn app<'a,'b>() -> App<'a,'b> {
    // Make Cargo.toml a dependency.
    let _ = include_str!("../Cargo.toml");
    app_from_crate!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("start")
                    .about("Create a new experiment context.")
                    .arg(Arg::with_name("name")
                         .index(1)
                         .help("Sets the name of the experiment.")
                         .required(true))
                    .arg(Arg::with_name("root")
                         .short("r")
                         .long("storage-root")
                         .help("The root under which experimental data is to be stored.")
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("file")
                    .about("Print the name of a file matching the given identifier.")
                    .arg(Arg::with_name("identifier")
                         .index(1)
                         .help("The data identifier for which to obtain a file.")
                         .required(true))
                    .arg(Arg::with_name("context")
                         .short("c")
                         .long("context")
                         .help("The experiment context.  Overrides EXPERIMENT_PATH.")
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("commit")
                    .about("Commit an experiment to its repository.")
                    .arg(Arg::with_name("root")
                         .short("r")
                         .long("storage-root")
                         .help("The root under which experimental data is to be stored.")
                         .takes_value(true))
                    .arg(Arg::with_name("context")
                         .short("c")
                         .long("context")
                         .help("The experiment context.  Overrides EXPERIMENT_PATH.")
                         .takes_value(true)))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_min() {
        app()
            .get_matches_from_safe(["experiment", "start", "name"].iter())
            .expect("'start' subcommand does not work.");
    }

    #[test]
    fn test_start_requires_experiment() {
        app()
            .get_matches_from_safe(["experiment", "start"].iter())
            .expect_err("No error despite no experiment name.");
    }

    #[test]
    fn test_start_accepts_root() {
        app()
            .get_matches_from_safe(["experiment", "start", "-r", "foo", "bar"].iter())
            .expect("Short '-r' not accepted.");

        app()
            .get_matches_from_safe(["experiment", "start", "--storage-root", "foo", "bar"].iter())
            .expect("Long '--storage-root' not accepted.");
    }

    #[test]
    fn test_file_min() {
        app()
            .get_matches_from_safe(["experiment", "file", "identifier"].iter())
            .expect("'file' subcommand does not work.");
    }

    #[test]
    fn test_file_requires_identifier() {
        app()
            .get_matches_from_safe(["experiment", "file"].iter())
            .expect_err("No error despite no identifier.");
    }
}
