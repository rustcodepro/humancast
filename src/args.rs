use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "solGenome",
    version = "1.0",
    about = "SolGenome.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    GenerateSol {
        // provide the path to the identified snps in personal genomics
        pathfile: String,
        // provide the specific snp for the token
        rsid: String,
    },
    GenerateInfo {
        // provide the rsid for the variant
        rsid: String,
    },
    AnnotateAll {
        // provide yes as a comment and it will annotate all variants
        annotate: String,
        // provide the path to the file
        pathfile: String,
    },
    RsIDSolana {
        // provide the rsid for the variant for which you want to generate a byte code
        rsid: String,
    },
}
