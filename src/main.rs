mod config;
mod schedule;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let login_uri = "https://bm5jadwal.azurewebsites.net/Auth/Login";
    let schedule_uri = "https://bm5jadwal.azurewebsites.net/Home/GetViconSchedule";

    // Fetch config or exit on unavailable config / unchanged cred
    let config = match config::read_config_or_exit().await {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1)
        }
    };

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Unable to create HTTP Client");

    // Form Body data
    let login_params = [
        ("Username", config.credentials.username),
        ("Password", config.credentials.password),
    ];

    let login_response = client.post(login_uri).form(&login_params).send().await?;
    println!("Login HTTP Status: {}", login_response.status());
    println!(
        "Login Response Body: {}\n",
        login_response.text().await.unwrap()
    );

    let schedule_response = client.get(schedule_uri).send().await?;
    // println!("Schedule Response: {}", schedule_response.text().await?);
    let schedule_text = schedule_response.text().await?;
    let schedule = serde_json::from_str::<schedule::Schedule>(&schedule_text)?;

    let current_time = chrono::Local::now();

    println!(" ======================");
    println!(" Today's Class Schedule");
    println!(" ======================");
    println!();

    for schedule_element in schedule.iter() {
        let combined_raw_end_schedule_time = format!(
            "{} {} +07:00",
            schedule_element.display_start_date, schedule_element.end_time
        );
        let schedule_end_time = chrono::DateTime::parse_from_str(
            combined_raw_end_schedule_time.as_str(),
            "%d %B %Y %H:%M:%S %:z",
        )?;

        // Break if schedule time is not today
        if schedule_end_time.date() != current_time.date() {
            break;
        }

        let already_finished = current_time > schedule_end_time;
        if already_finished {
            println!(
                "[✅] {} - {} ({})",
                schedule_element.start_time,
                schedule_element.course_title_en,
                schedule_element.delivery_mode,
            );
        } else {
            println!(
                "[▶] {} - {} ({})\n  {}",
                schedule_element.start_time,
                schedule_element.course_title_en,
                schedule_element.delivery_mode,
                schedule_element.meeting_url
            );
        }
    }

    // Open browser on upcoming class in 20 minutes (if any).
    // Don't forget to handle back to back classes.
    for schedule_element in schedule.iter().rev() {
        let combined_raw_start_schedule_time = format!(
            "{} {} +07:00",
            schedule_element.display_start_date, schedule_element.start_time
        );
        let schedule_start_time = chrono::DateTime::parse_from_str(
            combined_raw_start_schedule_time.as_str(),
            "%d %B %Y %H:%M:%S %:z",
        )?;

        let combined_raw_end_schedule_time = format!(
            "{} {} +07:00",
            schedule_element.display_start_date, schedule_element.end_time
        );
        let schedule_end_time = chrono::DateTime::parse_from_str(
            combined_raw_end_schedule_time.as_str(),
            "%d %B %Y %H:%M:%S %:z",
        )?;

        // Continue if schedule time is not today
        if schedule_end_time.date() != current_time.date() {
            continue;
        }

        // Conditionally open link; Break on first upcoming class
        if (current_time > (schedule_start_time - chrono::Duration::minutes(20)))
            && (current_time < schedule_end_time)
        {
            println!("\n\nOpening {}", &schedule_element.meeting_url);
            webbrowser::open(&schedule_element.meeting_url).unwrap();

            break;
        }
    }

    Ok(())
}
