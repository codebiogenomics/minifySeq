use crate::minimap::readfasta;
use crate::ontstruct::ONT;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-8-7
*/

pub async fn mapper(pathfile: &str, pathnos: &str) -> Result<String, Box<dyn Error>> {
    let mapfasta = readfasta(pathfile).unwrap();
    let mut filewrite = File::create("mapfasta.fasta").expect("file not present");
    for i in mapfasta.iter() {
        writeln!(filewrite, ">{}\t{}", i.id, i.sequence).expect("file not present");
    }
    let _ = Command::new("minimap")
        .arg("-a")
        .arg(pathfile)
        .arg(pathnos)
        .arg(">")
        .arg("mapped.sam")
        .output()
        .expect("commandfailed");

    // implimenting a hash structure for faster iteration match.
    let mut idhashet: HashSet<String> = HashSet::new();
    for i in mapfasta.iter() {
        idhashet.insert(i.id.clone());
    }

    let samparse: Vec<ONT> = Vec::new();
    let samopen = File::open("mapped.sam").expect("file not present");
    let samread = BufReader::new(samopen);
    // making a iterative vector for the search
    let mut ontsearch: Vec<(String, String)> = Vec::new();
    for i in samread.lines() {
        let line = i.expect("line not present");
        if !line.starts_with("@") {
            let linevec = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
            let vecinsert: (String, String) = (
                linevec[0].to_string(),
                linevec[1..linevec.len()].concat().to_string(),
            );
            ontsearch.push(vecinsert);
        }
    }

    Ok("Mapping has been finished".to_string())
}
