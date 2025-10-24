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
mod exp;
use crate::exp::read_csv;

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
        Commands::ThreadedLengthHuman { generate } => {
            let command = task::block_on(threadedlengthhuman(generate)).unwrap();
            println!("The command has finished:{:?}", command);
        }
        Commands::ThreadedLengthMouse { generate } => {
            let command = task::block_on(threadedlengthmouse(generate)).unwrap();
            println!("The command has finished:{:?}", command);
        }
        Commands::ThreadedHuman { count } => {
            let command = task::block_on(threadedlengthhuman(count)).unwrap();
            println!("The command has finished:{:?}", command);
        }
        Commands::ThreadedMouse { count } => {
            let command = task::block_on(threadedlengthmouse(count)).unwrap();
            println!("The command has finished:{:?}", command);
        }
        Commands::ExonThreadedHuman { count } => {
            let command = task::block_on(threadedlengthhumanexon(count)).unwrap();
            println!("The file has been written:{}", command);
        }
        Commands::ExonThreadedMouse { count } => {
            let command = task::block_on(threadedlengthmouseexon(count)).unwrap();
            println!("The file has been written:{}", command);
        }
        Commands::TMMEstimate { count } => {
            let input_path = Path::new(count);
            let output_path = input_path.with_extension("normalized.tsv");
            let mut data = read_csv(input_path)?;
            let (n_genes, n_samples) = data.dim();
            let lib_sizes = compute_library_sizes(&data);
            let ref_sample = 0;
            let tmm_factors = compute_tmm_factors(&data, &lib_sizes, ref_sample)?;
            let normalized = normalize_to_logcpm(&data, &lib_sizes, &tmm_factors)?;
            write_output(&normalized, &output_path)?;
            println!("Normalized data written to {}", output_path.display());
            Ok(())
        }
    }
}
