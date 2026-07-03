use KoalaNetty::net::client::Client;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut client = Client::connect(
        "ag6.ru:8000",
        "290ca313cc49bf44f92275373590956965c225b03095c30ef1".to_string(),
    )
    .await?;

    client.run().await?;

    Ok(())
}