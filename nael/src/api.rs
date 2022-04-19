use serde::Deserialize;

const BASE_VERSIONINFO_URL: &'static str = "https://kamori.goats.dev/Dalamud/Release/VersionInfo";
const BASE_ASSETS_URL: &'static str = "https://kamori.goats.dev/Dalamud/Asset/Meta";

#[derive(Deserialize, Debug)]
pub struct VersionInfoResponse {
    pub key: Option<String>,
    pub track: String,

    #[serde(rename = "assemblyVersion")]
    pub assembly_version: String,

    #[serde(rename = "runtimeVersion")]
    pub runtime_version: String,

    #[serde(rename = "runtimeRequired")]
    pub runtime_required: bool,

    #[serde(rename = "supportedGameVer")]
    pub supported_game_ver: String,

    #[serde(rename = "downloadUrl")]
    pub download_url: String,
}

#[derive(Deserialize, Debug)]
struct AssetChild {
    pub url: String,
    #[serde(rename="fileName")]
    pub file_name: String,
    pub hash: Option<String>, // We'll ignore this
}

#[derive(Deserialize, Debug)]
struct AssetInfoResponse {
    pub assets: Vec<AssetChild>,
}

#[derive(Debug)]
pub struct Asset {
    pub filename: String,
    pub url: String,
    pub hash: Option<String>,
}

pub async fn get_version_info(track: String) -> VersionInfoResponse {
    let client = reqwest::Client::new();
    let req = client.get(BASE_VERSIONINFO_URL)
        .query(&[("track", track)]);
    let resp = req.send().await.expect("failed to make http request");
    let text = resp.text().await.unwrap();
    let resp_json = serde_json::from_str::<VersionInfoResponse>(&text);
    resp_json.expect("bad JSON data!")
}

pub async fn get_assets_info() -> Vec<Asset> {
    let client = reqwest::Client::new();
    let req = client.get(BASE_ASSETS_URL);
    let resp = req.send().await.expect("failed to make http request");
    let text = resp.text().await.unwrap();
    let resp_json = serde_json::from_str::<AssetInfoResponse>(&text).expect("bad JSON!");
    resp_json.assets.iter().map(|x| {
        Asset {
            filename: x.file_name.clone(),
            url: x.url.clone(),
            hash: x.hash.clone()
        }
    }).collect::<Vec<Asset>>()
}