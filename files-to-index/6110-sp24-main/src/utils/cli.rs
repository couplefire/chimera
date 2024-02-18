/**
 * A generic command-line interface for 6.035 compilers.  This class
 * provides command-line parsing for student projects.  It recognizes
 * the required <tt>-target</tt>, <tt>-debug</tt>, <tt>-opt</tt>, and
 * <tt>-o</tt> switches, and generates a name for input and output
 * files.
 *
 * @author 6.1100 Staff, last updated January 2024
 */
use clap::Parser;

#[derive(Clone, clap::ValueEnum, Debug)]
pub enum CompilerAction {
    Default,
    Scan,
    Parse,
    Inter,
    Assembly,
}

#[derive(Clone, clap::ValueEnum, Debug, PartialEq, Eq, Hash)]
pub enum Optimization {}

#[derive(Parser, Debug)]
pub struct Args {
    /// compile to the given stage
    #[clap(short, long, value_enum, default_value_t=CompilerAction::Default, value_name = "stage")]
    pub target: CompilerAction,

    /// write output to
    #[clap(short, long, value_name = "outname")]
    pub output: Option<std::path::PathBuf>,

    /// Perform the listed optimizations
    #[clap(
        short = 'O',
        long,
        value_delimiter = ',',
        value_enum,
        value_name = "optimization,.."
    )]
    pub opt: Vec<Optimization>,

    /// Print debugging information
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Decaf file
    pub input: std::path::PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
