use crate::readstruct::ReadSNP;
use borsh::{BorshDeserialize, BorshSerialize};
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-8-18
*/

#[tokio::main]
pub async fn mapid(pathstring: &str, mapsnpid: String) -> Result<String, Box<dyn Error>> {
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

    let identifiedsnp = mapsnpid;
    let formatstring = format!("{}/{}", "https://www.ncbi.nlm.nih.gov/snp/", identifiedsnp);
    let response = get(formatstring).expect("string not found");
    let document = Html::parse_document(&response.text().expect("message not present"));
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
            .replace(":", "");
        println!("{:?}", vector_1);
    }

    Ok("path to the mapping string has been written".to_string())
}
