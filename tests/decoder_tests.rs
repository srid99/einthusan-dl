use einthusan_dl::scraper::decode_links;

#[test]
fn test_decode_links_valid_vector() {
    let payload = "eyJNUDRMaWturIjoiaHR0cHM6Ly9jZG4xLmVpbnRodXNhbi5pby9ldHYvY29udGVudC9EMlpFWS5tcDQ/ZT0xNzk0MzI0OTkwXHUwMDI2bWQ1PWItMm1WTDNIMWZoOVFXdVp5UkFrcmciLCJITFNMaW5rIjoiaHR0cHM6Ly9jZG4xLmVpbnRodXNhbi5pby9ldHYvY29udGVudC9EMlpFWS5tcDQubTN1OD9lPTE3OTQzMjQ5OTBcdTAwMjZtZDU9dllEeTNMWlJtMWdQNllpZnNPN0xnQSIsIkRhdGFjZW50ZXIiOiJXYXNoaW5ndG9uLCBELkMuIiwiUHJlbWl1bSI6ZmFsc2UsIlYiOjF95";
    let links = decode_links(payload).expect("should decode valid payload");

    assert_eq!(
        links.mp4_link,
        "https://cdn1.einthusan.io/etv/content/D2ZEY.mp4?e=1794324990&md5=b-2mVL3H1fh9QWuZyRAkrg"
    );
    assert_eq!(
        links.hls_link.as_deref(),
        Some("https://cdn1.einthusan.io/etv/content/D2ZEY.mp4.m3u8?e=1794324990&md5=vYDy3LZRm1gP6YifsO7LgA")
    );
    assert_eq!(links.datacenter.as_deref(), Some("Washington, D.C."));
    assert_eq!(links.premium, Some(false));
    assert_eq!(links.version, Some(1));
}

#[test]
fn test_decode_links_too_short() {
    let short_payload = "dG9vc2hvcnQ=";
    let err = decode_links(short_payload).unwrap_err();
    assert!(err.to_string().contains("too short"));
}

#[test]
fn test_decode_links_invalid_base64() {
    let invalid_b64 = "this_is_not_valid_base64_payload_at_all!!";
    let err = decode_links(invalid_b64).unwrap_err();
    assert!(err.to_string().contains("decode failed"));
}
