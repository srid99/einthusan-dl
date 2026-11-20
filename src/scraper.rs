use base64::prelude::*;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedLinks {
    #[serde(rename = "MP4Link")]
    pub mp4_link: String,
    #[serde(rename = "HLSLink")]
    pub hls_link: Option<String>,
    #[serde(rename = "Datacenter")]
    pub datacenter: Option<String>,
    #[serde(rename = "Premium")]
    pub premium: Option<bool>,
    #[serde(rename = "V")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct MoviePageMeta {
    pub page_id: String,
    pub ejpingables: String,
    pub movie_name: String,
    pub plot: Option<String>,
}

pub fn decode_links(value: &str) -> Result<DecodedLinks, AppError> {
    let len = value.len();
    if len < 13 {
        return Err(AppError::Decode(format!(
            "Input value too short for permutation: length {}",
            len
        )));
    }

    // Permutation logic matching Python: value[0:10] + value[value_len - 1] + value[12:value_len - 1]
    let part1 = &value[0..10];
    let last_char = &value[len - 1..len];
    let part2 = &value[12..len - 1];

    let mut encoded = String::with_capacity(len);
    encoded.push_str(part1);
    encoded.push_str(last_char);
    encoded.push_str(part2);

    let decoded_bytes = BASE64_STANDARD
        .decode(&encoded)
        .map_err(|e| AppError::Decode(format!("Base64 decode failed: {e}")))?;

    let decoded_str = String::from_utf8(decoded_bytes)
        .map_err(|e| AppError::Decode(format!("UTF-8 decode failed: {e}")))?;

    let links: DecodedLinks = serde_json::from_str(&decoded_str)
        .map_err(|e| AppError::Decode(format!("JSON parse of decoded value failed: {e}")))?;

    Ok(links)
}

pub fn parse_movie_page(html_str: &str) -> Result<MoviePageMeta, AppError> {
    let document = Html::parse_document(html_str);
    parse_movie_page_document(document, false)
}

pub fn parse_movie_page_with_plot(html_str: &str) -> Result<MoviePageMeta, AppError> {
    let document = Html::parse_document(html_str);
    parse_movie_page_document(document, true)
}

fn parse_movie_page_document(
    document: Html,
    include_plot: bool,
) -> Result<MoviePageMeta, AppError> {
    let html_selector = Selector::parse("html")
        .map_err(|e| AppError::Parse(format!("Invalid html selector: {e:?}")))?;
    let html_elem = document
        .select(&html_selector)
        .next()
        .ok_or_else(|| AppError::Parse("Missing <html> tag".into()))?;

    let page_id = html_elem
        .value()
        .attr("data-pageid")
        .ok_or_else(|| AppError::Parse("Missing data-pageid on <html>".into()))?
        .to_string();

    let player_selector = Selector::parse("section#UIVideoPlayer")
        .map_err(|e| AppError::Parse(format!("Invalid player selector: {e:?}")))?;
    let player_elem = document
        .select(&player_selector)
        .next()
        .ok_or_else(|| AppError::Parse("Missing section#UIVideoPlayer".into()))?;

    let ejpingables = player_elem
        .value()
        .attr("data-ejpingables")
        .ok_or_else(|| AppError::Parse("Missing data-ejpingables on section#UIVideoPlayer".into()))?
        .to_string();

    let summary_selector = Selector::parse("section#UIMovieSummary a.title h3")
        .map_err(|e| AppError::Parse(format!("Invalid summary selector: {e:?}")))?;
    let title_elem = document
        .select(&summary_selector)
        .next()
        .ok_or_else(|| AppError::Parse("Missing movie title in section#UIMovieSummary".into()))?;

    let movie_name = title_elem
        .text()
        .collect::<Vec<_>>()
        .join("")
        .trim()
        .to_string();

    let plot = if include_plot {
        let plot_selector = Selector::parse("section#UIMovieSummary p.synopsis")
            .map_err(|e| AppError::Parse(format!("Invalid plot selector: {e:?}")))?;
        document
            .select(&plot_selector)
            .next()
            .map(|elem| elem.text().collect::<Vec<_>>().join("").trim().to_string())
            .filter(|plot| !plot.is_empty())
    } else {
        None
    };

    Ok(MoviePageMeta {
        page_id,
        ejpingables,
        movie_name,
        plot,
    })
}

pub fn slugify_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify_filename("Jailer 2"), "Jailer_2");
        assert_eq!(slugify_filename("Movie! Name (2026)"), "Movie__Name__2026_");
    }

    #[test]
    fn test_decode_links() {
        let raw = "eyJNUDRMaWturIjoiaHR0cHM6Ly9jZG4xLmVpbnRodXNhbi5pby9ldHYvY29udGVudC9EMlpFWS5tcDQ/ZT0xNzk0MzI0OTkwXHUwMDI2bWQ1PWItMm1WTDNIMWZoOVFXdVp5UkFrcmciLCJITFNMaW5rIjoiaHR0cHM6Ly9jZG4xLmVpbnRodXNhbi5pby9ldHYvY29udGVudC9EMlpFWS5tcDQubTN1OD9lPTE3OTQzMjQ5OTBcdTAwMjZtZDU9dllEeTNMWlJtMWdQNllpZnNPN0xnQSIsIkRhdGFjZW50ZXIiOiJXYXNoaW5ndG9uLCBELkMuIiwiUHJlbWl1bSI6ZmFsc2UsIlYiOjF95";
        let res = decode_links(raw).expect("decode failed");
        assert!(res
            .mp4_link
            .starts_with("https://cdn1.einthusan.io/etv/content/D2ZEY.mp4"));
    }
}
