mod args;
mod proteome;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::proteome::Comprativedrive;
use crate::proteome::Endchain;
use crate::proteome::EuclideanWrite;
use crate::proteome::Pdbanalyze;
use crate::proteome::Pdbsequence;
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
        Commands::PDBId { pdbfile, output } => {
            let command = pdbchainwrite(pdbfile, output).unwrap();
            println!(
                "The atom coordinates for the following pdbf file are: {:?}",
                command
            );
        }
        Commands::PDBSequence { pdbfile } => {
            let command = pdbsequence(pdbfile);
            println!("The protein sequence is as follows: {:?}", command);
        }
        Commands::EuclideanComparative {
            pdbfile,
            chain,
            residue1,
            residue2,
            atom1,
            atom2,
        } => {
            let command =
                euclediancomparativetwo(pdbfile, chain, residue1, residue2, atom1, atom2).unwrap();
            println!(
                "The eucledian distance between to given coordinates of the same chain is {:?}",
                command
            );
        }
        Commands::EuclideanAll { pdbfile, chain } => {
            let command = eucledianall(pdbfile, chain).unwrap();
            println!(
                "The vector containing the eucleadian distance for those chain atoms are: {:?}",
                command
            );
        }
    }
}

fn pdbchainwrite(path: &str, output: &str) -> Result<String, Box<dyn Error>> {
    let pdbfile = File::open(path).expect("file not present");
    let pdbread = BufReader::new(pdbfile);
    let mut pdbanalyze: Vec<Pdbanalyze> = Vec::new();
    for i in pdbread.lines() {
        let line = i.expect("file not present");
        if line.starts_with("ATOM") {
            let linevec: Vec<_> = line
                .split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            if linevec.len() == 12usize{
            pdbanalyze.push(Pdbanalyze {
                adtype: linevec[0].to_string(),
                dnumber: linevec[1].to_string(),
                itype: linevec[2].to_string(),
                gtype: linevec[3].to_string(),
                ntype: linevec[4].to_string(),
                atype: linevec[5].to_string(),
                btype: linevec[6].to_string(),
                ctype: linevec[7].to_string(),
                utype: linevec[8].to_string(),
                attype: linevec[9].to_string(),
                lttype: linevec[10].to_string(),
                uchar: linevec[11].to_string(),
            });
            } else {
               continue
            }
        }
        }

    let mut filewrite = File::create(output).expect("file not present");
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

fn pdbsequence(path: &str) -> Result<String, Box<dyn Error>> {
    let mut pdbanalyze_amino: Vec<Pdbsequence> = Vec::new();
    let mut pdbanalyze_ter: Vec<Endchain> = Vec::new();
    let pdbfile = File::open(path).expect("file not present");
    let pdbread = BufReader::new(pdbfile);
    for i in pdbread.lines() {
        let line = i.expect("file not present");
        if line.starts_with("ATOM") {
            let linevec: Vec<_> = line
                .split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            if linevec.len() == 12usize {
            pdbanalyze_amino.push(Pdbsequence {
                adtype: linevec[0].to_string(),
                dnumber: linevec[1].to_string(),
                gtype: linevec[3].to_string(),
                ntype: linevec[4].to_string(),
                atype: linevec[5].to_string(),
                btype: linevec[6].to_string(),
                ctype: linevec[7].to_string(),
                utype: linevec[8].to_string(),
                attype: linevec[9].to_string(),
                lttype: linevec[10].to_string(),
                uchar: linevec[11].to_string(),
            });
            }
        } else if line.starts_with("TER") {
            let lineadapt: Vec<_> = line
                .split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            pdbanalyze_ter.push(Endchain {
                tab0: lineadapt[2].to_string(),
            });
        } else {
             continue
        }
    }
    let mut sequence: Vec<String> = Vec::new();
    for i in pdbanalyze_amino.iter() {
        sequence.push(i.gtype.clone())
    }
    for i in pdbanalyze_ter.iter() {
        sequence.push(i.tab0.to_string());
    }

    let aminoacid: Vec<(String, String)> = vec![
        ("ALA".to_string(), "A".to_string()),
        ("ARG".to_string(), "R".to_string()),
        ("ASN".to_string(), "N".to_string()),
        ("ASP".to_string(), "D".to_string()),
        ("CYS".to_string(), "C".to_string()),
        ("GLU".to_string(), "E".to_string()),
        ("GLN".to_string(), "Q".to_string()),
        ("GLY".to_string(), "G".to_string()),
        ("HIS".to_string(), "H".to_string()),
        ("ILE".to_string(), "I".to_string()),
        ("LEU".to_string(), "L".to_string()),
        ("LYS".to_string(), "K".to_string()),
        ("MET".to_string(), "M".to_string()),
        ("PHE".to_string(), "F".to_string()),
        ("PRO".to_string(), "P".to_string()),
        ("SER".to_string(), "S".to_string()),
        ("THR".to_string(), "T".to_string()),
        ("TRP".to_string(), "W".to_string()),
        ("TYR".to_string(), "Y".to_string()),
        ("VAL".to_string(), "V".to_string()),
    ];

    let mut finalsequence: Vec<String> = Vec::new();

    for i in sequence.iter() {
        for j in aminoacid.iter() {
            if *i == *j.0 {
                let value = j.1.clone();
                finalsequence.push(value);
            }
        }
    }

    Ok(finalsequence.join("").to_string())
}

fn euclediancomparativetwo(
    path: &str,
    chain: &str,
    residue1: &str,
    residue2: &str,
    atom1: &str,
    atom2: &str,
) -> Result<f32, Box<dyn Error>> {
    let residue1_arg = residue1.parse::<usize>().unwrap();
    let residue2_arg = residue2.parse::<usize>().unwrap();
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut comparative1: Vec<Comprativedrive> = Vec::new();
    let mut comparative2: Vec<Comprativedrive> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not found");
        if line.starts_with("ATOM") {
            let mutline = line
                .split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            if mutline[4] == chain
                && mutline[5].parse::<usize>().unwrap() == residue1_arg
                && mutline[2] == atom1
            {
                comparative1.push(Comprativedrive {
                    chain: mutline[4].to_string(),
                    residue: mutline[5].parse::<usize>().unwrap(),
                    coordinate1: mutline[6].parse::<f32>().unwrap(),
                    coordinate2: mutline[7].parse::<f32>().unwrap(),
                    coordinate3: mutline[8].parse::<f32>().unwrap(),
                });
            } else if mutline[4] == chain
                && mutline[5].parse::<usize>().unwrap() == residue2_arg
                && mutline[2] == atom2
            {
                comparative2.push(Comprativedrive {
                    chain: mutline[4].to_string(),
                    residue: mutline[5].parse::<usize>().unwrap(),
                    coordinate1: mutline[6].parse::<f32>().unwrap(),
                    coordinate2: mutline[7].parse::<f32>().unwrap(),
                    coordinate3: mutline[8].parse::<f32>().unwrap(),
                })
            }
        }
    }

    let mut coordinate_euclidean: f32 = 0.0f32;
    for i in comparative1.iter() {
        for j in comparative2.iter() {
            let eucledian1 = (i.coordinate1 - j.coordinate1).powi(2);
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
        if line.starts_with("ATOM") {
            let mutline = line
                .split(" ")
                .filter(|x| !x.is_empty())
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
    }

    let mut coordinate_euclidean_all: Vec<f32> = Vec::new();
    for i in 0..coordinate.len() - 1 {
        let mut euclidean = 0.0f32;
        let eucledian1 = (coordinate[i].0 - coordinate[i + 1].0).powi(2);
        let eucledian2 = (coordinate[i].1 - coordinate[i + 1].1).powi(2);
        let eucledian3 = (coordinate[i].2 - coordinate[i + 1].2).powi(2);
        euclidean += (eucledian1 + eucledian2 + eucledian3).sqrt();
        coordinate_euclidean_all.push(euclidean)
    }

    let mut coordinate_euclidean_write_all: Vec<Vec<EuclideanWrite>> = Vec::new();
    for i in 0..coordinate.len() - 1 {
        let mut euclidean = 0.0f32;
        let euclidean1 = (coordinate[i].0 - coordinate[i + 1].0).powi(2);
        let eucledian2 = (coordinate[i].1 - coordinate[i + 1].1).powi(2);
        let eucledian3 = (coordinate[i].2 - coordinate[i + 1].2).powi(2);
        euclidean += (euclidean1 + eucledian2 + eucledian3).sqrt();
        let mut eucledian_vec: Vec<EuclideanWrite> = Vec::new();
        eucledian_vec.push(EuclideanWrite {
            tab1: coordinate[i].0,
            tab2: coordinate[i].1,
            tab3: coordinate[i].2,
            tab4: coordinate[i + 1].0,
            tab5: coordinate[i + 1].1,
            tab6: coordinate[i + 1].2,
            tab7: euclidean,
        });
        coordinate_euclidean_write_all.push(eucledian_vec);
    }

    let mut filewrite = File::create("Eucledian-distance-all.txt").expect("file not present");
    for i in coordinate_euclidean_write_all.iter() {
        for j in i.iter() {
            writeln!(
                filewrite,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                j.tab1, j.tab2, j.tab3, j.tab4, j.tab5, j.tab6, j.tab7
            )
            .expect("file not found");
        }
    }

    Ok(coordinate_euclidean_all)
}
