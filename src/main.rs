use reqwest::Client;

const YEAR: u32 = 2024;
const URL: &str = "https://adventofcode.com/2024";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let response = reqwest::get(URL).await?;
    let html = response.text().await?;

    for day in 1..=25 {
        let input_file = format!("./inputs/day{day}.txt");
        let src_file = format!("./examples/day{day}.rs");
        let day_html = day_str(YEAR, day);

        if !html.contains(&day_html) {
            break;
        }

        if std::fs::exists(&input_file)? {
            continue;
        }

        println!("{URL}/day/{day}");

        let url = format!("{URL}/day/{day}/input");
        let session = std::env::var("SESSION")?;
        let response = Client::builder()
            .build()?
            .get(url)
            .header("Cookie", format!("session={session}"))
            .send()
            .await?;

        let content = response.text().await?;
        
        let src = format!(r#"const INPUT: &str = include_str!("../inputs/day{day}.txt");

fn main() {{
    
}}
"#);

        std::fs::write(&input_file, &content)?;
        std::fs::write(&src_file, &src)?;
    }

    Ok(())
}

fn day_str(year: u32, day: u32) -> String {
    format!(r#"<a aria-label="Day {day}" href="/{year}/day/{day}" class="calendar-day{day}">"#)
}
