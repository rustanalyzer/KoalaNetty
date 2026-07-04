use KoalaNetty::net::client::client::Client;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut client = Client::connect(
        "ag6.ru:8000",
        "".to_string(),
    )
    .await?;

    client.start().await?;

    client.send_message(12729, "Привет").await?;
    client.api().friends().await?;

    client.run().await?;

    Ok(())
}