mod annotate;
mod args;
mod generatannot;
mod map;
mod readstruct;
mod threadedhuman;
mod threadedhumanexon;
mod threadedmouse;
mod threadedmouseexon;
use crate::annotate::annotateall;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::exp::compute_tmm_factors;
use crate::generatannot::getrsid;
use crate::map::mapid;
use crate::rsidfetch::rsidfetch;
use crate::threadedhuman::threadedlengthhuman;
use crate::threadedhumanexon::threadedlengthhumanexon;
use crate::threadedmouse::threadedlengthmouse;
use crate::threadedmouseexon::threadedlengthmouseexon;
use clap::Parser;
use figlet_rs::FIGfont;
mod exp;
use crate::exp::read_csv;

/*
Gaurav Sablok
codeprog@icloud.com
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("genomicsSOL");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::MaprsID {
            pathfile,
            rsid,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = mapid(pathfile, rsid.to_string()).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::GenerateInfo { rsid, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = getrsid(rsid).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::AnnotateAll {
            annotate,
            pathfile,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = annotateall(annotate, pathfile).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::ThreadedLengthHuman { generate, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = threadedlengthhuman(generate);
                println!("The command has finished:{:?}", command);
            });
        }
        Commands::ThreadedLengthMouse { generate, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = threadedlengthmouse(generate).unwrap();
                println!("The command has finished:{:?}", command);
            });
        }
        Commands::ThreadedHuman { count, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = threadedlengthhuman(count).unwrap();
                println!("The command has finished:{:?}", command);
            });
        }
        Commands::ThreadedMouse { count, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = threadedlengthmouse(count).unwrap();
                println!("The command has finished:{:?}", command);
            });
        }
        Commands::ExonThreadedHuman { count, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = threadedlengthhumanexon(count).unwrap();
                println!("The file has been written:{}", command);
            });
        }
        Commands::ExonThreadedMouse { count, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = threadedlengthmouseexon(count).unwrap();
                println!("The file has been written:{}", command);
            });
        }
    }
}
