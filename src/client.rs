use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use std::time::Duration;
use tracing::{debug, info};

use crate::error::AppError;
use crate::scraper::{
    decode_links, parse_movie_page, parse_movie_page_with_plot, DecodedLinks, MoviePageMeta,
};

const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

#[derive(Debug, Deserialize)]
struct AjaxResponse {
    #[serde(rename = "Data")]
    data: Option<AjaxData>,
}

#[derive(Debug, Deserialize)]
struct AjaxData {
    #[serde(rename = "EJLinks")]
    ej_links: Option<String>,
}

pub struct EinthusanClient {
    client: reqwest::Client,
}

impl EinthusanClient {
    pub fn new() -> Result<Self, AppError> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));

        let client = reqwest::Client::builder()
            .cookie_store(true)
            .default_headers(headers)
            .connect_timeout(Duration::from_secs(15))
            .read_timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self { client })
    }

    pub async fn fetch_page(&self, url: &str) -> Result<String, AppError> {
        debug!("Fetching page from URL: {}", url);
        let resp = self.client.get(url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(AppError::Parse(format!(
                "HTTP request for {} returned status {}",
                url, status
            )));
        }

        let body = resp.text().await?;
        info!("Downloaded {} ({} bytes)", url, body.len());
        Ok(body)
    }

    pub async fn resolve_movie_info(
        &self,
        movie_page_url: &str,
    ) -> Result<(MoviePageMeta, DecodedLinks), AppError> {
        self.resolve_movie_info_with_plot(movie_page_url, false)
            .await
    }

    pub async fn resolve_movie_info_with_plot(
        &self,
        movie_page_url: &str,
        include_plot: bool,
    ) -> Result<(MoviePageMeta, DecodedLinks), AppError> {
        let html = self.fetch_page(movie_page_url).await?;
        let meta = if include_plot {
            parse_movie_page_with_plot(&html)?
        } else {
            parse_movie_page(&html)?
        };

        let ajax_url = movie_page_url.replace("movie", "ajax/movie");
        let x_json = serde_json::json!({
            "EJOutcomes": meta.ejpingables,
            "NativeHLS": false
        })
        .to_string();

        let params = [
            ("xEvent", "UIVideoPlayer.PingOutcome"),
            ("xJson", &x_json),
            ("gorilla.csrf.Token", &meta.page_id),
        ];

        debug!("Posting ping outcome to {}", ajax_url);
        let resp = self.client.post(&ajax_url).form(&params).send().await?;

        let status = resp.status();
        if !status.is_success() {
            return Err(AppError::Parse(format!(
                "AJAX request to {} returned status {}",
                ajax_url, status
            )));
        }

        let text = resp.text().await?;
        let ajax_res: AjaxResponse = serde_json::from_str(&text).map_err(|e| {
            AppError::Parse(format!(
                "Failed to parse AJAX response JSON: {e} (response: {text})"
            ))
        })?;

        let ej_links = ajax_res
            .data
            .and_then(|d| d.ej_links)
            .ok_or_else(|| AppError::Parse("AJAX response missing Data.EJLinks".into()))?;

        let decoded = decode_links(&ej_links)?;
        Ok((meta, decoded))
    }
}
