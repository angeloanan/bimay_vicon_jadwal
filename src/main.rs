mod config;
mod schedule;

use indoc::{formatdoc, indoc};
use url::Url;

#[derive(serde::Deserialize)]
struct ZoomUrlQuery {
    pwd: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let financial_blocker_uri =
        "https://apim-bm7-prod-web.azure-api.net/func-bm7-notification-prod/FinancialBlocker";
    let schedule_uri = "https://apim-bm7-prod-web.azure-api.net/func-bm7-course-prod/ClassSession/Upcoming/student";

    // Fetch config or exit on unavailable config / unchanged cred
    let config = match config::read_config_or_exit().await {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1)
        }
    };

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; bimay_vicon_jadwal/0.1; +https://github.com/angeloanan/bimay_vicon_jadwal)")
        .cookie_store(true)
        .build()
        .expect("Unable to create HTTP Client");

    // HTTP headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", config.token).parse().unwrap(),
    );

    // Fetch financial blocker
    // This is basically used to verify whether the user is logged in or not
    let financial_blocker_response = client
        .get(financial_blocker_uri)
        .headers(headers.to_owned())
        .send()
        .await?;

    println!(
        "Financial Blocker HTTP Status: {}",
        financial_blocker_response.status()
    );

    if !financial_blocker_response.status().is_success() {
        panic!(indoc! {"
            > Invalid token (unable to fetch financial blocker)
            Please check the config file whether you have the right token for NewBinusmaya. Token might change randomly due to it expiring.
        "});
    }

    println!(
        "Login Response Body: {}\n",
        financial_blocker_response.text().await.unwrap()
    );

    // Fetch upcoming class
    let upcoming_class_response = client
        .get(schedule_uri)
        .headers(headers.to_owned())
        .send()
        .await?;
    let upcoming_class_text = upcoming_class_response.text().await?;
    let upcoming_class_data =
        serde_json::from_str::<schedule::UpcomingClass>(&upcoming_class_text)?;

    if upcoming_class_data.join_url.is_none() {
        println!("Your upcoming class does not have a Zoom URL. Please check NewBinusmaya.");
        std::process::exit(0);
    }

    println!(
        "\n\nOpening Zoom meeting for {}",
        &upcoming_class_data.course_name
    );

    let zoom_url = Url::parse(&upcoming_class_data.join_url.to_owned().unwrap()).expect(
        formatdoc! {"
            > Unable to parse URL
            We are unable to parse the Zoom URL. Please manually join the meeting: {}
        ", upcoming_class_data.join_url.unwrap()
        }
        .as_str(),
    );

    let zoom_meeting_id = zoom_url
        .path_segments()
        .unwrap()
        .nth(1)
        .unwrap()
        .to_string();
    let zoom_meeting_password = serde_qs::from_str::<ZoomUrlQuery>(zoom_url.query().unwrap())
        .unwrap()
        .pwd;

    // https://medium.com/zoom-developer-blog/zoom-url-schemes-748b95fd9205
    open::that(format!(
        "zoommtg://zoom.us/join?confno={}&pwd={}&zc=0&uname={}",
        zoom_meeting_id, zoom_meeting_password, config.zoom_username
    ))
    .unwrap();

    // TODO: Migrate this to NewBinusmaya
    // let schedule_response = client.get(schedule_uri).send().await?;
    // // println!("Schedule Response: {}", schedule_response.text().await?);
    // let schedule_text = schedule_response.text().await?;
    // let schedule = serde_json::from_str::<schedule::Schedule>(&schedule_text)?;

    // let current_time = chrono::Local::now();

    // println!(" ======================");
    // println!(" Today's Class Schedule");
    // println!(" ======================");
    // println!();

    // for schedule_element in schedule.iter() {
    //     let combined_raw_end_schedule_time = format!(
    //         "{} {} +07:00",
    //         schedule_element.display_start_date, schedule_element.end_time
    //     );
    //     let schedule_end_time = chrono::DateTime::parse_from_str(
    //         combined_raw_end_schedule_time.as_str(),
    //         "%d %B %Y %H:%M:%S %:z",
    //     )?;

    //     // Break if schedule time is not today
    //     if schedule_end_time.date() != current_time.date() {
    //         break;
    //     }

    //     let already_finished = current_time > schedule_end_time;
    //     if already_finished {
    //         println!(
    //             "[✅] {} - {} ({})",
    //             schedule_element.start_time,
    //             schedule_element.course_title_en,
    //             schedule_element.delivery_mode,
    //         );
    //     } else {
    //         println!(
    //             "[▶] {} - {} ({})\n  {}",
    //             schedule_element.start_time,
    //             schedule_element.course_title_en,
    //             schedule_element.delivery_mode,
    //             schedule_element.meeting_url
    //         );
    //     }
    // }

    // // Open browser on upcoming class in 20 minutes (if any).
    // // Don't forget to handle back to back classes.
    // for schedule_element in schedule.iter().rev() {
    //     let combined_raw_start_schedule_time = format!(
    //         "{} {} +07:00",
    //         schedule_element.display_start_date, schedule_element.start_time
    //     );
    //     let schedule_start_time = chrono::DateTime::parse_from_str(
    //         combined_raw_start_schedule_time.as_str(),
    //         "%d %B %Y %H:%M:%S %:z",
    //     )?;

    //     let combined_raw_end_schedule_time = format!(
    //         "{} {} +07:00",
    //         schedule_element.display_start_date, schedule_element.end_time
    //     );
    //     let schedule_end_time = chrono::DateTime::parse_from_str(
    //         combined_raw_end_schedule_time.as_str(),
    //         "%d %B %Y %H:%M:%S %:z",
    //     )?;

    //     // Continue if schedule time is not today
    //     if schedule_end_time.date() != current_time.date() {
    //         continue;
    //     }

    //     // Conditionally open link; Break on first upcoming class
    //     if (current_time > (schedule_start_time - chrono::Duration::minutes(20)))
    //         && (current_time < schedule_end_time)
    //     {
    //         // If no link on upcoming class, then just skip
    //         if schedule_element.meeting_id != "-" {
    //             println!(
    //                 "\n\nOpening Zoom meeting for {}",
    //                 &schedule_element.course_title_en
    //             );

    //             // https://medium.com/zoom-developer-blog/zoom-url-schemes-748b95fd9205
    //             open::that(format!(
    //                 "zoommtg://zoom.us/join?confno={}&pwd={}&zc=0&uname={} - {}",
    //                 schedule_element.meeting_id,
    //                 schedule_element.meeting_password,
    //                 schedule_element.person_code,
    //                 capitalize_first_letter(&schedule_element.full_name)
    //             ))
    //             .unwrap();
    //         }

    //         break;
    //     }
    // }

    Ok(())
}

// Input a sentence and make every first letter of the word capitalized
fn capitalize_first_letter(sentence: &str) -> String {
    let mut result = String::new();

    for word in sentence.to_ascii_lowercase().split_whitespace() {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            result.push(first.to_ascii_uppercase());
            result.extend(chars);
        }

        result.push(' ');
    }

    result.pop();
    result
}
