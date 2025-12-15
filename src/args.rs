use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "humanCAST",
    version = "1.0",
    about = "humanCAST: human genomics conversion
       ************************************************
       Gaurav Sablok
       codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    MaprsID {
        // provide the path to the identified snps in personal genomics
        pathfile: String,
        // provide the specific snp for the token
        rsid: String,
        /// threads for the analysis
        thread: String,
    },
    GenerateInfo {
        // provide the rsid for the variant
        rsid: String,
        /// thread for the analysis
        thread: String,
    },
    AnnotateAll {
        // provide yes as a comment and it will annotate all variants
        annotate: String,
        // provide the path to the file
        pathfile: String,
        /// threads for the analysis
        thread: String,
    },
    /// threaded version of genelength for human
    ThreadedLengthHuman {
        /// provide yes as argument
        generate: String,
        /// thread for the analysis
        thread: String,
    },
    /// trhreaded version of genelength for mouse
    ThreadedLengthMouse {
        /// provide yes as argument
        generate: String,
        /// thread for the analysis
        thread: String,
    },
    /// threaded version of genelength for human
    ThreadedHuman {
        /// provide yes as argument
        count: String,
        /// thread for the analysis
        thread: String,
    },
    /// threaded version of genelength for mouse
    ThreadedMouse {
        /// provide yes as argument
        count: String,
        /// thread for the analysis
        thread: String,
    },
    /// only exon coverage
    ExonThreadedHuman {
        /// provide yes as argument
        count: String,
        /// thread for the analysis
        thread: String,
    },
    /// only exon coverage
    ExonThreadedMouse {
        /// provide yes as argument
        count: String,
        /// thread for the analysis
        thread: String,
    },
}
