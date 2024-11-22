use std::{net::SocketAddr, path::Path, time::Duration};

use reqwest::Client;
use tokio::fs;

use crate::{
    model::{FileRequest, FileResponse, UploadParam},
    server::ServerSetting,
};

pub async fn send_register(
    setting: &ServerSetting,
    addr: &SocketAddr,
) -> Result<(), reqwest::Error> {
    let url = format!("http://{}/api/localsend/v2/register", addr.to_string());
    Client::new()
        .post(url)
        .body(serde_json::json!(setting.to_device_message(None)).to_string())
        .timeout(Duration::from_millis(100))
        .send()
        .await?;
    Ok(())
}

pub async fn prepare_upload(
    file_req: &FileRequest,
    addr: &SocketAddr,
) -> Result<FileResponse, reqwest::Error> {
    let url = format!(
        "http://{}/api/localsend/v2/prepare_upload",
        addr.to_string()
    );
    Client::new()
        .post(url)
        .body(serde_json::json!(file_req).to_string())
        .timeout(Duration::from_secs(60))
        .send()
        .await?
        .json()
        .await
}

pub async fn upload(
    upload_param: UploadParam,
    file_path: &Path,
    addr: &SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "http://{}/api/localsend/v2/upload?sessionId={}&fileId={}&token={}",
        addr.to_string(),
        upload_param.session_id,
        upload_param.file_id,
        upload_param.token
    );
    let file = fs::read(file_path).await?;
    Client::new().post(url).body(file).send().await?;
    Ok(())
}

pub async fn cancel(sessionId: String, addr: &SocketAddr) -> Result<(), reqwest::Error> {
    let url = format!(
        "http://{}/api/localsend/v2/cancel?sessionId={}",
        addr.to_string(),
        sessionId
    );
    Client::new().post(url).send().await?;
    Ok(())
}
