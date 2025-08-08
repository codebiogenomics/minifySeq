use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "minifySeq",
    version = "1.0",
    about = "RUST-CLEAN.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// remove ONT
    Threadedremoval {
        /// provide ONT file
        pathfile: String,
        /// path denos file
        denosfile: String,
    },
}
