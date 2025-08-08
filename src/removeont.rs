use crate::minimap::readfasta;
use crate::ontstruct::ONT;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-7-16
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

    Ok("Mapping has been finished".to_string())
}
