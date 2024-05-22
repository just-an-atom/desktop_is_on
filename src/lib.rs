use dotenv::dotenv;
use gethostname::gethostname;
use reqwest::Error;

pub async fn post_it() -> Result<(), Error> {
    dotenv().ok();

    // Config Data
    let _pc_name = gethostname();

    let url = std::env::var("POST_URL").expect("POST_URL is not set.");
    let json_data = "{\"title\": \"".to_owned()
        + { _pc_name.to_str().expect("Hostname not valid string") }
        + " is online!\"}";

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status: {}", response.status());

    let response_body = response.text().await?;
    println!("Response body:\n{}", response_body);

    Ok(())
}
