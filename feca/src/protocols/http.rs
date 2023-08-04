use url::Url;

pub async fn get(url: &Url) -> anyhow::Result<String> {
    let resp = reqwest::get(url.as_str()).await?;

    Ok(resp.text().await?)
}
