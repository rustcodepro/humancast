use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-7-16
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct GeneLength {
    startvec: usize,
    endvec: usize,
    length: usize,
}

pub async fn threadedlengthmouse(generate: &str) -> Result<String, Box<dyn Error>> {
    let pathadd = Path::new("gencode.v48.primary_assembly.annotation.gtf.gz");
    if generate == "yes" && !pathadd.exists() {
        let _ = Command::new("wget").
         arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_mouse/release_M37/gencode.vM37.primary_assembly.annotation.gtf.gz")
         .output()
            .expect("command failed");
        let _ = Command::new("gunzip")
            .arg("gencode.vM37.primary_assembly.annotation.gtf.gz")
            .output()
            .expect("Command failed");
        let _ = Command::new("gunzip")
            .arg("*")
            .output()
            .expect("Command failed");

        let fileall =
            File::open("gencode.vM37.primary_assembly.annotation.gtf").expect("file not present");
        let filecontent = BufReader::new(fileall);
        let mut id: Vec<String> = Vec::new();
        let mut start: Vec<usize> = Vec::new();
        let mut end: Vec<usize> = Vec::new();
        for i in filecontent.lines() {
            let line = i.expect("line not present");
            if !line.starts_with("#") {
                let linevec: Vec<_> = line
                    .split("\t")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                for _i in 0..linevec.len() {
                    if linevec[2].clone().to_owned() == "gene" {
                        start.push(linevec[3].parse::<usize>().unwrap());
                        end.push(linevec[4].parse::<usize>().unwrap());
                        id.push(
                            linevec[8].split(";").collect::<Vec<_>>()[0]
                                .to_string()
                                .replace(" ", "-")
                                .split("-")
                                .collect::<Vec<_>>()[1]
                                .to_string(),
                        )
                    }
                }
            }
        }
        let mut genevecrwrite: HashMap<String, GeneLength> = HashMap::new();
        for i in 0..id.len() {
            genevecrwrite.insert(
                id[i].clone(),
                GeneLength {
                    startvec: start[i],
                    endvec: end[i],
                    length: end[i] - start[i],
                },
            );
        }

        let mut filewrite = File::create("genelength.txt").expect("File not present");
        for (i, j) in genevecrwrite.iter() {
            writeln!(
                filewrite,
                "{}\t{}\t{}\t{}",
                i, j.startvec, j.endvec, j.length
            )
            .expect("no file present");
        }

        let _ = Command::new("rm")
            .arg("-rf")
            .arg("gencode.vM37.primary_assembly.annotation.gtf")
            .output()
            .expect("file not found");

        Ok("The gene length have been written".to_string())
    } else {
        Ok("no option supplied".to_string())
    }
}
