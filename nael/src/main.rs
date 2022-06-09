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

    #[clap(short='n', long="--dotnet", help="Download the .NET runtime as well")]
    pub dotnet: bool,
}

async fn download_file(url: &str) -> reqwest::Result<Vec<u8>> {
    let bytes = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    Ok(bytes.into_iter().collect::<Vec<u8>>())
}

async fn simple_download_file(url: &str, res_path: &std::path::Path) -> anyhow::Result<()> {
    let file_name = res_path.file_name().unwrap().to_string_lossy();
    print!("Downloading {} ({})... ", file_name, url);
    std::io::stdout().flush().unwrap();
    let resp = download_file(url).await;
    match resp {
        Ok(data) => {
            std::fs::write(res_path, data).unwrap();
            println!("okay");
        },
        Err(x) => {
            println!("NG");
            return Err(anyhow::Error::from(x));
        },
    }

    Ok(())
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
                    println!("NG");
                    panic!("FAILED to download file: {}", x)
                },
            }
        }
        return;
    }

    let resp = api::get_version_info(cli.track.unwrap_or("release".into())).await;

    println!("Downloading Dalamud v.{}", resp.assembly_version);
    println!("(For game version '{}', this will probably not work with another game version)", resp.supported_game_ver);

    let output_path = std::env::current_dir().unwrap();
    let output_path = output_path.join(format!("dalamud-latest-{}.zip", resp.assembly_version));
    simple_download_file(&resp.download_url, &output_path)
        .await
        .expect("FAILED to download Dalamud");

    if cli.dotnet {
        if resp.runtime_required {
            let rt = resp.runtime_version;
            println!("Downloading .NET runtime v.{}", rt);
            let dotnet_url = format!("https://kamori.goats.dev/Dalamud/Release/Runtime/DotNet/{}", rt);
            let dotnet_desktop_url = format!("https://kamori.goats.dev/Dalamud/Release/Runtime/WindowsDesktop/{}", rt);
            let current_dir = std::env::current_dir().unwrap();
            let dotnet_path = current_dir.join(format!("dotnet-runtime-{}.zip", rt));
            let dotnet_desktop_path = current_dir.join(format!("dotnet-desktop-{}.zip", rt));
            simple_download_file(&dotnet_url, &dotnet_path)
                .await
                .expect("FAILED to download .NET runtime");
            simple_download_file(&dotnet_desktop_url, &dotnet_desktop_path)
                .await
                .expect("FAILED to download .NET desktop runtime");
        } else {
            println!("Runtime is not required for this build of Dalamud. Not doing anything.");
        }
    }

    println!("All done. Please watch out for Iron Chariot next time.");
}
