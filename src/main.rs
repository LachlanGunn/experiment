// Author: Lachlan Gunn
//
// Copyright:
//   2018 Secure Systems Group, Aalto University https://ssg.aalto.fi/
//
// This code is released under the Apache 2.0 licence.

#[macro_use] extern crate clap;
extern crate chrono;
extern crate data_encoding;
extern crate failure;
#[macro_use] extern crate failure_derive;
extern crate rand;
extern crate tempfile;

mod cli;

use std::io::Read;
use std::io::Write;

use rand::Rng;

#[derive(Fail,Debug)]
enum ExperimentAppError {
    #[fail(display = "No experiment context specified by --context or EXPERIMENT_PATH.")]
    NoContext,
    #[fail(display = "Failed to create ancestor directories.")]
    AncestorCreation,
    #[fail(display = "Failed to create file.")]
    FileCreation,
    #[fail(display = "Invalid experimental context.")]
    InvalidContext,
    #[fail(display = "No storage root specified by --storage-root or EXPERIMENT_STORAGE_ROOT.")]
    NoStorageRoot,
    #[fail(display = "Invalid experimental storage repository.")]
    InvalidStorageRoot,
}

fn main() -> Result<(), failure::Error> {
    match run() {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), failure::Error> {
    let matches = cli::app().get_matches();

    if let Some(matches) = matches.subcommand_matches("start") {
        // Get input values.
        let storage_path =
            match matches.value_of("root") {
                Some(p) => std::path::PathBuf::from(p),
                None => match std::env::var("EXPERIMENT_STORAGE_ROOT") {
                    Ok(p) => std::path::PathBuf::from(p),
                    Err(_) => Err(ExperimentAppError::NoStorageRoot)?
                }
            };

        let context_name = matches.value_of("name").expect("Required argument not present.");

        // Get the ROOT/new path (creating the directory if needed)
        let new_path = storage_path.join(".new");
        if !new_path.is_dir() {
            std::fs::create_dir_all(&new_path)
                .map_err(|_| ExperimentAppError::InvalidStorageRoot)?;
        }

        // Now create the temporary path.
        let context_path = tempfile::tempdir_in(new_path)?.into_path();

        {
            let mut name_file = std::fs::File::create(&context_path.join("name"))?;
            name_file.write_all(context_name.as_bytes())?;
        }

        {
            let time = chrono::Utc::now();
            let mut timestamp_file = std::fs::File::create(&context_path.join("start-time"))?;
            timestamp_file.write_all(time.format("%+").to_string().as_bytes())?;
        }

        println!("{}", context_path.to_string_lossy());
    }
    else if let Some(matches) = matches.subcommand_matches("file") {
        // First get the experiment context. This can come from several sources,
        // in order of priority:
        //
        //   1. The --context command-line parameter.
        //   2. EXPERIMENT_PATH environmental variable.
        let context_path =
            match matches.value_of("context") {
                Some(p) => std::path::PathBuf::from(p),
                None => match std::env::var("EXPERIMENT_PATH") {
                    Ok(p) => std::path::PathBuf::from(p),
                    Err(_) => Err(ExperimentAppError::NoContext)?
                }
            };

        // At the moment we just use the path directly, but eventually we will
        // want to extract more meaning from the identifier.
        let internal_path = std::path::PathBuf::from(
            matches
            .value_of("identifier")
                .expect("Required argument not present."));

        // We then join the internal path to the experimental path, yielding the
        // file.
        let file_path = context_path.join(internal_path);

        // Next we must create the directory containing the file-to-be
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)
                .or_else(|_| Err(ExperimentAppError::AncestorCreation))?;
        }

        // Create the file itself.
        std::fs::File::create(&file_path)
            .or_else(|_| Err(ExperimentAppError::FileCreation))?;

        // Print the filename to stdout.
        println!("{}", file_path.to_string_lossy());
    }
    else if let Some(matches) = matches.subcommand_matches("commit") {
        // First get the experiment context. This can come from several sources,
        // in order of priority:
        //
        //   1. The --context command-line parameter.
        //   2. EXPERIMENT_PATH environmental variable.
        let context_path =
            match matches.value_of("context") {
                Some(p) => std::path::PathBuf::from(p),
                None => match std::env::var("EXPERIMENT_PATH") {
                    Ok(p) => std::path::PathBuf::from(p),
                    Err(_) => Err(ExperimentAppError::NoContext)?
                }
            };

        // Next, find where to put the experiment.
        let storage_path =
            match matches.value_of("root") {
                Some(p) => std::path::PathBuf::from(p),
                None => match std::env::var("EXPERIMENT_STORAGE_ROOT") {
                    Ok(p) => std::path::PathBuf::from(p),
                    Err(_) => Err(ExperimentAppError::NoContext)?
                }
            };

        // Read the experiment name/timestamp
        let experiment_name = {
            let mut name_file = match std::fs::File::open(context_path.join("name")) {
                Ok(f) => f,
                Err(_) => Err(ExperimentAppError::InvalidContext)?,
            };
            let mut name = String::new();
            name_file.read_to_string(&mut name)?;
            name
        };

        let experiment_timestamp = {
            let mut timestamp_file = match std::fs::File::open(context_path.join("start-time")) {
                Ok(f) => f,
                Err(_) => Err(ExperimentAppError::InvalidContext)?,
            };
            let mut timestamp = String::new();
            timestamp_file.read_to_string(&mut timestamp)?;
            timestamp
        };

        // We now have all the information that we need to produce the final
        // location for the experimental data.
        let final_experimental_path = {
            let mut random_code = [0u8; 4];
            loop {
                // Produce a random disambiguating code.
                rand::thread_rng().try_fill(&mut random_code)?;

                // Construct the path.
                let experiment_dir = format!("{}-{}-{}",
                                         experiment_name,
                                         experiment_timestamp,
                                         data_encoding::HEXLOWER.encode(
                                             &random_code[..]));

                let path_proposal = storage_path.join(experiment_dir);

                // Check whether the path exists.  If not, we are done.
                if !path_proposal.exists() {
                    break path_proposal;
                }
            }
        };

        std::fs::rename(context_path, final_experimental_path)?;
    }

    Ok(())
}
