use crate::{cli::*, cli_types::*, commands::ProveCommand, errors::CLIError};

use snarkos_algorithms::snark::verify_proof;

use clap::ArgMatches;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct RunCommand;

impl CLI for RunCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Run a program with inputs";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "run";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<(), CLIError> {
        let (proof, prepared_verifying_key) = ProveCommand::output(options)?;

        let mut verifying = Duration::new(0, 0);

        let start = Instant::now();

        let is_success = verify_proof(&prepared_verifying_key, &proof, &vec![]).unwrap();

        verifying += start.elapsed();

        println!(" ");
        println!("  Verifier time   : {:?} milliseconds", verifying.as_millis());
        println!("  Verifier output : {}", is_success);
        println!(" ");

        Ok(())
    }
}
