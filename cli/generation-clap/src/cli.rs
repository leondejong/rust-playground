use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Random Sample Generator")]
#[command(author = "Author <name@domain.tld")]
#[command(version = "1.0")]
#[command(about = "Generate random integers, floats and strings", long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate random float
    Float {},
    /// Generate random integer
    Integer {
        // Signed (flag)
        #[arg(short, long)]
        sign: bool,
    },
    /// Generate random string
    String {
        /// Length (integer)
        #[arg(short, long)]
        length: Option<usize>,
    },
    /// Generate random ranged float
    FloatRange {
        /// Start value (float)
        #[arg(short, long)]
        start: f64,
        /// End value (float)
        #[arg(short, long)]
        end: f64,
    },
    /// Generate random ranged integer
    IntegerRange {
        /// Start value (integer)
        #[arg(short, long)]
        start: i64,
        /// End value (integer)
        #[arg(short, long)]
        end: i64,
    },
    /// Generate random custom sample string
    Custom {
        /// Length (integer)
        #[arg(short, long)]
        length: usize,
        /// Characters (string)
        #[arg(short, long)]
        chars: String,
    },
}
