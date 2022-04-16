use clap::StructOpt;

mod api;

#[derive(clap::Parser)]
struct CommandLine {
    #[clap(short='t', long="--track", help="Dalamud track to download")]
    pub track: String,
}

#[tokio::main]
async fn main() {
    let cli = CommandLine::parse();
    let resp = api::get_version_info(cli.track).await;
    println!("{:#?}", resp);
}
