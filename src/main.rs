mod args;
mod genome;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::genome::Comprativedrive;
use crate::genome::Pdbanalyze;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
* Author Gaurav Sablok
* Date: 2025-1-29
* Universitat Potsdam and SLB Potsdam
*
* Set of the evolutionary genome analysis tools written
* for the genomeand the proteome analysis.

* */

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::PDBId { pdbfile } => {
            let command = pdbchainwrite(pdbfile).unwrap();
            println!(
                "The atom coordinates for the following pdbf file are: {:?}",
                command
            );
        }
        Commands::PDBSequence { pdbfile, seq } => {
            let command = pdbsequence(pdbfile, seq.clone()).unwrap();
            println!("The protein sequence is as follows: {:?}", command);
        }
        Commands::EucledianComparative {
            pdbfile,
            chain,
            residue1,
            residue2,
        } => {
            let command = euclediancomparativetwo(pdbfile, chain, residue1, residue2).unwrap();
            println!(
                "The eucledian distance between to given coordinates of the same chain is {:?}",
                command
            );
        }
        Commands::EucledianAll { pdbfile, chain } => {
           let command = eucledianall(pdbfile, chain).unwrap();
           println!("The vector containing the eucleadian distance for those chain atoms are: {:?}", command);
        }
    }
}

fn pdbchainwrite(path: &str) -> Result<String, Box<dyn Error>> {
    let pdbfile = File::open(path).expect("file not present");
    let pdbread = BufReader::new(pdbfile);
    let mut pdbanalyze: Vec<Pdbanalyze> = Vec::new();
    for i in pdbread.lines() {
        let line = i.expect("file not present");
        if line.starts_with("ATOM") {
            let linevec: Vec<_> = line.split("\t").collect::<Vec<_>>();
            pdbanalyze.push(Pdbanalyze {
                adtype: linevec[0].to_string(),
                dnumber: linevec[1].parse::<usize>().unwrap(),
                itype: linevec[2].to_string(),
                gtype: linevec[3].to_string(),
                atype: linevec[4].parse::<usize>().unwrap(),
                ntype: linevec[5].to_string(),
                btype: linevec[6].parse::<usize>().unwrap(),
                ctype: linevec[7].parse::<usize>().unwrap(),
                utype: linevec[8].parse::<usize>().unwrap(),
                attype: linevec[9].parse::<usize>().unwrap(),
                lttype: linevec[10].parse::<usize>().unwrap(),
                uchar: linevec[11].to_string(),
            })
        }
    }

    let filename = format!(
        "{:?}{:?}",
        path.split(".").collect::<Vec<_>>()[0].to_string(),
        ".txt"
    );

    let mut filewrite = File::create(filename).expect("file not present");
    for i in pdbanalyze.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.adtype,
            i.dnumber,
            i.itype,
            i.gtype,
            i.atype,
            i.ntype,
            i.btype,
            i.ctype,
            i.utype,
            i.attype,
            i.lttype,
            i.uchar
        )
        .expect("line not found");
    }

    Ok("the table delimited file has been written with the coordinates".to_string())
}

fn pdbsequence(path: &str, seq: Option<String>) -> Result<String, Box<dyn Error>> {
    if seq.unwrap() == *"1" {
        let pdbfile = File::open(path).expect("file not present");
        let pdbread = BufReader::new(pdbfile);
        let mut pdbanalyze: Vec<Pdbanalyze> = Vec::new();
        for i in pdbread.lines() {
            let line = i.expect("file not present");
            if line.starts_with("ATOM") {
                let linevec: Vec<_> = line.split("\t").collect::<Vec<_>>();
                pdbanalyze.push(Pdbanalyze {
                    adtype: linevec[0].to_string(),
                    dnumber: linevec[1].parse::<usize>().unwrap(),
                    itype: linevec[2].to_string(),
                    gtype: linevec[3].to_string(),
                    atype: linevec[4].parse::<usize>().unwrap(),
                    ntype: linevec[5].to_string(),
                    btype: linevec[6].parse::<usize>().unwrap(),
                    ctype: linevec[7].parse::<usize>().unwrap(),
                    utype: linevec[8].parse::<usize>().unwrap(),
                    attype: linevec[9].parse::<usize>().unwrap(),
                    lttype: linevec[10].parse::<usize>().unwrap(),
                    uchar: linevec[11].to_string(),
                })
            }
        }
        let mut sequence: Vec<String> = Vec::new();
        for i in pdbanalyze.iter() {
            sequence.push(i.gtype.clone())
        }
    }

    Ok("the sequence has been written".to_string())
}

fn euclediancomparativetwo(
    path: &str,
    chain: &str,
    residue1: &str,
    residue2: &str,
) -> Result<f32, Box<dyn Error>> {
    let residue1_arg = residue1.parse::<usize>().unwrap();
    let residue2_arg = residue2.parse::<usize>().unwrap();
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut comparative1: Vec<Comprativedrive> = Vec::new();
    let mut comparative2: Vec<Comprativedrive> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not found");
        let mutline = line
            .split("\t")
            .filter(|x| x.is_empty())
            .collect::<Vec<_>>();
        if mutline[4] == chain && mutline[5].parse::<usize>().unwrap() == residue1_arg {
            comparative1.push(Comprativedrive {
                chain: mutline[4].to_string(),
                residue: mutline[5].parse::<usize>().unwrap(),
                coordinate1: mutline[6].parse::<f32>().unwrap(),
                coordinate2: mutline[7].parse::<f32>().unwrap(),
                coordinate3: mutline[8].parse::<f32>().unwrap(),
            });
        } else if mutline[4] == chain && mutline[5].parse::<usize>().unwrap() == residue2_arg {
            comparative2.push(Comprativedrive {
                chain: mutline[4].to_string(),
                residue: mutline[5].parse::<usize>().unwrap(),
                coordinate1: mutline[6].parse::<f32>().unwrap(),
                coordinate2: mutline[7].parse::<f32>().unwrap(),
                coordinate3: mutline[8].parse::<f32>().unwrap(),
            })
        }
    }

    let mut coordinate_euclidean: f32 = 0.0f32;
    for i in comparative1.iter() {
        for j in comparative2.iter() {
            let eucledian1 = (i.coordinate1 - j.coordinate2).powi(2);
            let eucledian2 = (i.coordinate2 - j.coordinate2).powi(2);
            let eucledian3 = (i.coordinate3 - j.coordinate3).powi(2);
            coordinate_euclidean += (eucledian1 + eucledian2 + eucledian3).sqrt();
        }
    }

    Ok(coordinate_euclidean)
}

fn eucledianall(path: &str, chain: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    let pathopen = File::open(path).expect("file not present");
    let pathread = BufReader::new(pathopen);
    let mut coordinate: Vec<(f32, f32, f32)> = Vec::new();
    for i in pathread.lines() {
        let line = i.expect("file not found");
        let mutline = line
            .split("\t")
            .filter(|x| x.is_empty())
            .collect::<Vec<_>>();
        if mutline[4] == chain {
            let tuplevec = (
                mutline[6].parse::<f32>().unwrap(),
                mutline[7].parse::<f32>().unwrap(),
                mutline[8].parse::<f32>().unwrap(),
            );
            coordinate.push(tuplevec);
        }
    }

    let mut coordinate_euclidean_all: Vec<f32> = Vec::new();
    for i in 0..coordinate.len()-1{
      let mut euclidean = 0.0f32;
       let eucledian1 = (coordinate[i].0 - coordinate[i+1].0).powi(2);
            let eucledian2 = (coordinate[i].1 - coordinate[i+1].1).powi(2);
            let eucledian3 = (coordinate[i].2 - coordinate[i+1].2).powi(2);
            euclidean += (eucledian1 + eucledian2 + eucledian3).sqrt();
            coordinate_euclidean_all.push(euclidean)
    }

    Ok(coordinate_euclidean_all)
}
