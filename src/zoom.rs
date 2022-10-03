use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct ZoomUrlQuery {
    pwd: String,
}

/// Opens a meeting in Zoom
///
/// Protocol documentation: https://medium.com/zoom-developer-blog/zoom-url-schemes-748b95fd9205
///
/// Note: Blogpost might have been removed, use wayback machine!
pub fn open_meeting(id: &str, pwd: &str, username: Option<&str>) {
    let mut uri = format!("zoommtg://zoom.us/join?confno={}&pwd={}&zc=0", id, pwd);

    if let Some(username) = username {
        uri.push_str(&format!("&uname={}", urlencoding::encode(username)));
    }

    open::that(uri).unwrap()
}

/// Parses a zoom meeting link (https://), returning their Meeting ID and hashed password
pub fn parse_meeting_url(url: &str) -> (String, String) {
    let zoom_url = Url::parse(url).expect(&format!("Invalid zoom meeting url: {}", url));

    let zoom_meeting_id = zoom_url
        .path_segments()
        .unwrap()
        .nth(1)
        .unwrap()
        .to_string();

    let zoom_meeting_password = serde_qs::from_str::<ZoomUrlQuery>(zoom_url.query().unwrap())
        .unwrap()
        .pwd;

    (zoom_meeting_id, zoom_meeting_password)
}
