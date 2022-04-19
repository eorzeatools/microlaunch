use clap::StructOpt;

mod api;

use ring::digest::{Context, Digest, SHA1_FOR_LEGACY_USE_ONLY};
use std::io::{Read, Write};

pub fn sha1_digest<R: Read>(mut reader: R) -> Result<Digest, std::io::Error> {
    let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

#[derive(clap::Parser)]
struct CommandLine {
    #[clap(short='t', long="--track", help="Dalamud track to download")]
    pub track: Option<String>,

    #[clap(short='a', long="--assets", help="Download assets")]
    pub assets: bool,
}

async fn download_file(url: &str) -> reqwest::Result<Vec<u8>> {
    let bytes = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    Ok(bytes.into_iter().collect::<Vec<u8>>())
}

#[tokio::main]
async fn main() {
    let cli = CommandLine::parse();

    if cli.assets {
        let assets_resp = api::get_assets_info().await;
        match std::fs::create_dir("assets") {
            Ok(_) => {},
            Err(x) => {
                match x.kind() {
                    std::io::ErrorKind::AlreadyExists => {},
                    _ => panic!("IO error: {}", x)
                }
            },
        }
        for i in assets_resp {
            print!("Downloading {} ({})... ", &i.filename, &i.url);
            std::io::stdout().flush().unwrap();
            let resp = download_file(&i.url).await;
            match resp {
                Ok(data) => {
                    let output_path = std::path::Path::new("assets");
                    let output_path = output_path.join(&i.filename);
                    std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
                    std::fs::write(&output_path, data).expect("Error while writing file!!");
                    // Hash check
                    if let Some(hash) = &i.hash {
                        let file = std::fs::File::open(output_path).unwrap();
                        let file_hash = sha1_digest(file).expect("Error while getting digest");
                        let orig_hash = data_encoding::HEXUPPER.decode(hash.as_bytes()).expect("Bad hash orig");
                        let refed = file_hash.as_ref();
                        if orig_hash != refed {
                            // HASH MISMATCH!! SOUND THE ALARM, ABORT, ABANDON SHIP
                            println!("HASH MISMATCH");
                            let refed_str = data_encoding::HEXUPPER.encode(refed);
                            panic!("HASH MISMATCH on file {} - expected {} but got {}", &i.filename, hash, refed_str);
                        }
                    }
                    println!("okay");
                },
                Err(x) => {
                    panic!("FAILED to download file: {}", x)
                },
            }
        }
        return;
    }

    let resp = api::get_version_info(cli.track.expect("Want a track when downloading main")).await;
    println!("{:#?}", resp);
}
