use KoalaNetty::net::client::client::Client;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut client = Client::connect(
        "ag6.ru:8000",
        "3eca20b3ffbfc13e1026e1f370ec3ef13f2e99d877876a5d21".to_string(),
    )
    .await?;

    client.start().await?;

    client.api().nick_change("꧁ ๖ۣۣۜᴅɪᴍᴜsʜᴋᴀ.     *em46* ꧂").await?;
    client.send_message(12729, "Привет").await?;

    client.run().await?;

    Ok(())
}