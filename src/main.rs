mod model;

use std::{env, fs, time::Instant};

use anyhow::{anyhow, bail, Result};

use model::*;

fn main() -> Result<()> {
    let args: Vec<_> = env::args_os().collect();
    if args.len() != 3 {
        bail!("Usage: ./modlinks-sha-verifier <api|mod> <path-to-xml-file>");
    }

    println!("Reading XML file");
    let buf = fs::read(&args[2])
        .map_err(|e| anyhow!("Failed to read XML file\n::error title=XML Read Error::{e}"))?;

    let doc: Box<dyn Verify> = match args[1].to_str() {
        Some("api") => {
            println!("Deserializing ApiLinks document");
            Box::new(
                quick_xml::de::from_slice::<'_, ApiLinks>(&buf).map_err(|e| {
                    anyhow!(
                    	"Failed to deserialize ApiLinks XML file\n::error title=XML Deserialization Error::{e}"
                	)
                })?,
            )
        }
        Some("mod") => {
            println!("Deserializing ModLinks document");
            Box::new(
                quick_xml::de::from_slice::<'_, ModLinks>(&buf).map_err(|e| {
                    anyhow!(
                    	"Failed to deserialize ModLinks XML file\n::error title=XML Deserialization Error::{e}"
                	)
                })?,
            )
        }
        _ => bail!("Document type must be api or mod"),
    };

    rayon::ThreadPoolBuilder::new()
        //Since network speed is not a problem in the production environment, this *can* be very high
        .num_threads(256)
        .build_global()
        .unwrap();
    let client = ureq::agent();

    println!("Starting verification\n");
    println!("{:-<111}", "");

    let start_time = Instant::now();
    let res = doc.verify(&client);
    let duration = start_time.elapsed();

    println!("{:-<111}", "");
    println!(
        "\nDone in {:0>2}:{:0>2}.{:0>3}",
        duration.as_secs() / 60,
        duration.as_secs() % 60,
        duration.as_millis() % 1000
    );

    match res {
        true => println!("Verification PASSED"),
        false => bail!("Verification FAILED"),
    };

    Ok(())
}
