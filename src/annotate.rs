use crate::readstruct::ReadAnnotate;
use crate::readstruct::ReadSNP;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn annotateall(optionannot: &str, pathstring: &str) -> Result<String, Box<dyn Error>> {
    if optionannot == "yes" && pathstring != "" {
        let filesnpopen = File::open(pathstring).expect("file not present");
        let filesnpread = BufReader::new(filesnpopen);

        let mut opensnpstruct: Vec<ReadSNP> = Vec::new();

        for i in filesnpread.lines() {
            let line = i.expect("line not present");
            if !line.starts_with("#") {
                let linestore = line
                    .split("\t")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                opensnpstruct.push(ReadSNP {
                    rsid: linestore[0].clone(),
                    chromosome: linestore[1].clone(),
                    position: linestore[2].clone(),
                    genotype: linestore[3].clone(),
                })
            }
        }

        let mut snpvec: Vec<(String, String)> = Vec::new();
        for i in opensnpstruct.iter() {
            let linestruct = format!("{}/{}", "https://www.ncbi.nlm.nih.gov/snp/", i.rsid);
            snpvec.push((i.rsid.clone(), linestruct));
        }

        let mut asyncvec: Vec<ReadAnnotate> = Vec::new();
        thread::spawn(move || {
            for i in snpvec.iter() {
                for j in opensnpstruct.iter() {
                    if i.0 == j.rsid {
                        let response = get(i.1.clone()).expect("string not found");
                        let document =
                            Html::parse_document(&response.text().expect("message not present"));
                        let snpselect = Selector::parse(".sect_heading").expect("method failed");
                        for element in document.select(&snpselect) {
                            let vector_1 = element
                                .text()
                                .collect::<Vec<_>>()
                                .join("-")
                                .replace(" ", "")
                                .replace("\n", "")
                                .replace("Genomecontext", "")
                                .replace("Selectflanklength", "")
                                .replace(":", "")
                                .replace("GenomicPlacements", "")
                                .replace("\"\"", "");
                            asyncvec.push(ReadAnnotate {
                                rsid: j.rsid.clone(),
                                chromosome: j.chromosome.clone(),
                                position: j.position.clone(),
                                genotype: j.genotype.clone(),
                                annotate: vector_1,
                            });
                        }
                    }
                }
            }
        });
    }
    Ok("path to the mapping string has been written".to_string())
}
