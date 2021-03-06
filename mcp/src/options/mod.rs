pub fn _output_formats() -> &'static [&'static str] {
    &["json", "yaml"]
}

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(setting = structopt::clap::AppSettings::VersionlessSubcommands)]
#[structopt(setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
pub struct Args {
    /// The desired log level.
    #[structopt(short = "l", long = "log-level", default_value = "INFO")]
    #[structopt(possible_values = &["INFO", "ERROR", "DEBUG", "TRACE"])]
    pub log_level: log::Level,
    #[structopt(subcommand)]
    pub(crate) cmd: SubCommand,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    #[structopt(name = "fetch-api-specs")]
    /// Fetch all API specs, in parallel, and generate their Rust code
    FetchApiSpecs(fetch_specs::Args),
    #[structopt(name = "completions")]
    /// generate completions for supported shells
    Completions(completions::Args),
    #[structopt(name = "generate")]
    /// generate APIs and CLIs for a Google API specification
    Generate(generate::Args),
    #[structopt(name = "map-api-index")]
    /// Transform the API index into data we can use during substitution
    MapApiIndex(map_index::Args),
    #[structopt(name = "substitute")]
    #[structopt(alias = "sub")]
    /// Substitutes templates using structured data.
    Substitute(substitute::Args),
    #[structopt(name = "cargo-errors")]
    /// Run cargo on workspace files and collect errors as we go
    CargoErrors(cargo_errors::Args),
}

pub mod cargo_errors;
pub mod completions;
pub mod fetch_specs;
pub mod generate;
pub mod map_index;
pub mod substitute;
