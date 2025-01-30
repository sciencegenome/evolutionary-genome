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
        output: String,
    },
    /// extract the sequence of the pdf file
    PDBSequence {
       pdbfile: String,
    },
    /// calculate the euclidean distance bettwen two chain coordinates
    EuclideanComparative{
     /// provide the pdb file
     pdbfile: String,
     /// provide the chain to be selected
     chain: String,
     /// provide the residue 1
     residue1: String,
     /// provide the resiude 2
     residue2: String,
     /// provide the atom1
     atom1: String,
     /// provide the atom2
     atom2: String,
    },
    /// calculates the euclidean distance for all chain atoms
    EuclideanAll {
     /// provide the pdb file
     pdbfile: String,
     /// provide the chain
     chain: String,
    }
}
