mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use async_std::task::block_on;
use clap::Parser;
mod minimap;
use async_std::task;
mod ontstruct;
mod removeont;
use crate::removeont::mapper;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-7-16
*/

fn main() {
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::Threadedremoval {
            pathfile,
            denosfile,
        } => {
            let command = task::block_on(mapper(pathfile, denosfile)).unwrap();
            println!("The command has finished:{}", command);
        }
    }
}
