use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "evolutionary-proteome",
    version = "1.0",
    about = "set of evolutionary analysis for proteome"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    PDBId {
        /// please provide the path to the pdb file
        pdbfile: String,
    },
    /// extract the sequence of the pdf file
    PDBSequence {
       pdbfile: String,
       seq: Option<String>,
    },
    /// calculate the euclidean distance bettwen two chain coordinates
    EucledianComparative{
     /// provide the pdb file
     pdbfile: String,
     /// provide the chain to be selected
     chain: String,
     /// provide the residue 1
     residue1: String,
     /// provide the resiude 2
     residue2: String,
    }
}
