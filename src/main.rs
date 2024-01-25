use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(unused)]
struct Server {
    name: String,
    ip: String,
    id: String,
    ami: String,
    size: String,
    subnet: String,
    hostname: String
}

#[derive(Parser, Debug)]
struct Args {
    query: String,
    #[structopt(short, long, default_value = "3389")]
    port: u32,
    #[structopt(short, long, default_value = "servers.csv")]
    csv: String
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    // Load the CSV
    let file = std::fs::File::open(args.csv).context("servers.csv does not exist, are you in the correct directory?")?;
    let buf = std::io::BufReader::new(file);
    let mut reader = csv::Reader::from_reader(buf);
    let mut servers = vec!();
    for result in reader.deserialize() {
        let result: Server = result.context("Bad row")?;
        servers.push(result);
    }

    // Perform the search, get first arg
    let query = std::env::args().nth(1).context("No query")?;
    let mut server = None;
    for s in servers.iter() {
        if s.hostname.contains(&query) {
            server = Some(s);
            break;
        }
    }
    let server = server.context("No server found")?;

    println!("Connecting to {}, {} on {}", server.name, server.ip, &args.port);
    
    // Start an AWS CLI session

    //ssm start-session --document-name AWS-StartPortForwardingSession --parameters '{"portNumber": ["3389"], "localPortNumber": ["3390"]}' --target
    let mut child = Command::new("aws")
        .arg("ssm")
        .arg("start-session")
        .arg("--document-name")
        .arg("AWS-StartPortForwardingSession")
        .arg("--parameters")
        .arg(format!(r#"{{"portNumber": ["3389"], "localPortNumber": ["{}"]}}"#, args.port))
        .arg("--target")
        .arg(&server.id)
        .stdout(Stdio::inherit()) 
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to start AWS CLI")?;

    let status = child.wait().expect("Failed to wait on child");

    println!("Process exited with status: {:?}", status);

    Ok(())
}
