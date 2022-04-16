use serde::Deserialize;

const BASE_VERSIONINFO_URL: &'static str = "https://kamori.goats.dev/Dalamud/Release/VersionInfo";

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

pub async fn get_version_info(track: String) -> VersionInfoResponse {
    let client = reqwest::Client::new();
    let req = client.get(BASE_VERSIONINFO_URL)
        .query(&[("track", track)]);
    let resp = req.send().await.expect("failed to make http request");
    let text = resp.text().await.unwrap();
    let resp_json = serde_json::from_str::<VersionInfoResponse>(&text);
    resp_json.expect("bad JSON data!")
}