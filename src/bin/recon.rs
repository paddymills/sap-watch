
use clap::Parser;
use std::error::Error;
use sap_watch::db::Sndb;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    part: Option<String>,

    #[arg(short, long)]
    prev_week: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    sap_watch::logging::init_logger();
    
    let args = Args::parse();
    let mut sn = Sndb::init().await?;
    
    if let Some(part) = args.part {
        let qty = sn.get_part_burned_qty(&part).await?;
        println!("{}: {}", part, qty);
    };

    if args.prev_week {
        let week = sn.get_parts_burned_for_week().await?;
        for x in &week {
            println!("{:?}", x);
        }

        let progs: std::collections::HashSet<&String> = week.iter().map(|x| &x.program).collect();
        println!("Programs: {}", progs.len());
    }

    Ok(())
}
