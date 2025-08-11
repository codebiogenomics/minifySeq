use crate::minimap::FASTA;
use crate::minimap::readfasta;
use crate::ontstruct::Clean;
use async_std::task;
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
    // added async here
    let mapfasta = task::block_on(readfasta(pathfile)).unwrap();
    let mut filewrite = File::create("mapfasta.fasta").expect("file not present");
    for i in mapfasta.iter() {
        writeln!(filewrite, ">{}\t{}", i.id, i.sequence).expect("file not present");
    }
    // mapping using the minimap algorithm
    let _ = Command::new("minimap")
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

    let mut cleanvec: Vec<Clean> = Vec::new();
    let mut hashvecid = HashSet::new();
    for i in ontsearch.iter() {
        hashvecid.insert(i.0.clone());
    }
    // hashing the arrays in a hashset
    for i in hashvecid.iter() {
        let mut startvec: Vec<usize> = Vec::new();
        let mut endvec: Vec<usize> = Vec::new();
        for j in ontsearch.iter() {
            if j.0.clone().to_string() == *i {
                let splitvec: Vec<String> =
                    j.1.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
                startvec.push(splitvec[2].parse::<usize>().unwrap());
                endvec.push(splitvec[3].parse::<usize>().unwrap());
            }
        }
        cleanvec.push(Clean {
            id: i.to_string(),
            start: startvec,
            end: endvec,
        })
    }

    // iterative clipping
    let mut vecstorage: Vec<FASTA> = Vec::new();
    for i in mapfasta.iter() {
        for j in 0..cleanvec.len() {
            let mut vecsec = String::from("");
            if i.id == cleanvec[j].id {
                let vecseq = String::from(i.sequence.clone());
                for seq in 0..cleanvec[j].start.len() {
                    let newseq = &vecseq[cleanvec[j].start[seq]..cleanvec[j].end[seq]];
                    vecsec.push_str(newseq);
                }
            }
            vecstorage.push(FASTA {
                id: i.id.clone(),
                sequence: vecsec.to_string(),
            })
        }
    }

    // stats for the merged count

    let mut count: usize = 0usize;
    let mut countgap: usize = 0usize;
    for i in ontsearch.iter() {
        let basecount = i.1.split("\t").collect::<Vec<_>>()[9]
            .parse::<usize>()
            .unwrap();
        count += basecount;
        let countg = i.1.split("\t").collect::<Vec<_>>()[10]
            .parse::<usize>()
            .unwrap();
        countgap += countg;
    }

    let mut veccount: Vec<_> = Vec::new();
    let mut vecseq: Vec<usize> = Vec::new();
    for i in ontsearch.iter() {
        veccount.push(i.0.clone());
        vecseq.push(i.1.len());
    }
    let writesec = vecseq.iter().fold(0, |acc, x| acc + x);
    let mut statfile = File::create("statfile.txt").expect("file not present");
    writeln!(statfile, "{}", "The stats for the given file are:").expect("file not present");
    writeln!(
        statfile,
        "Number of sequences:{}\tTotal bases:{}\tNumber of matching bases:{}\tNumber of bases including gaps:{}",
        veccount.len(),
        writesec.to_string(),
        count,
        countgap
    ).expect("file not written");

    Ok("Mapping has been finished".to_string())
}
