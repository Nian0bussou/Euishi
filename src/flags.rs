use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// sort the files ; if both move & scramble are provided scramble will be used first
    Move_ {
        /// provide the path
        #[arg(short, long)]
        path: Option<String>,

        /// specify if you want to choose which dir to act on
        #[arg(short, long)]
        chooseDirs: bool,
    },

    /// scramble the files ; if both move & scramble are provided scramble will be used first
    Scramble {
        /// provide the path
        #[arg(short, long)]
        path: Option<String>,

        /// specify if you want to choose which dir to act on
        #[arg(short, long)]
        chooseDirs: bool,
    },

    /// remove tmp files
    Remove {
        /// provide the path
        #[arg(short, long)]
        path: Option<String>,
        /// will print each file when using removeTmps
        #[arg(short, long)]
        verbose: bool,
    },

    /// self explanatory
    Skibiditoiletrizzinohiofrfrbrainrot,
}
