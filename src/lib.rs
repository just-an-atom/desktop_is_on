use gethostname::gethostname;
use reqwest::Error;

pub async fn post_it() -> Result<(), Error> {
    // Config Data
    let pc_name = gethostname();
    let pc_name_str = pc_name.to_str().expect("Hostname not valid string");

    // Use compile-time environment variable
    let url = env!("POST_URL");
    let json_data = format!("{{\"title\": \"🟢 {} is now online!\"}}", pc_name_str);

    println!("Debug Info: POST_URL = {}", url); // Debug print
    println!("Debug Info: JSON Data = {}", json_data); // Debug print

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_data)
        .send()
        .await?;

    println!("Status: {}", response.status());

    let response_body = response.text().await?;
    println!("Response body:\n{}", response_body);

    Ok(())
}
