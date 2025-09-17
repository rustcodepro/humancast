mod annotate;
mod args;
mod createsolana;
mod generatannot;
mod map;
mod readstruct;
use crate::annotate::annotateall;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::createsolana::getrsidsolana;
use crate::generatannot::getrsid;
use crate::map::mapid;
use clap::Parser;
use figlet_rs::FIGfont;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-8-7
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("genomicsSOL");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::GenerateSol { pathfile, rsid } => {
            let command = mapid(pathfile, rsid.to_string()).unwrap();
            println!("The command has finished:{}", command);
        }
        Commands::GenerateInfo { rsid } => {
            let command = getrsid(rsid).unwrap();
            println!("The command has finished:{}", command);
        }
        Commands::AnnotateAll { annotate, pathfile } => {
            let command = annotateall(annotate, pathfile).unwrap();
            println!("The command has finished:{}", command);
        }
        Commands::RsIDSolana { rsid } => {
            let command = getrsidsolana(rsid).unwrap();
            println!("The borsh index for the token has been written:{}", command);
        }
    }
}
