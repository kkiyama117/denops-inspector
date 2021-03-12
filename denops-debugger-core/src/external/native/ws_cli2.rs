use futures_util::{future, SinkExt, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;

// create single-thread client and send messages.
pub async fn run_client(url: &str) -> anyhow::Result<()> {
    let (c, res) = connect_async(url).await?;
    let (mut writer, mut reader) = c.split();

    // create thread to manage sending message
    let write_thread = tokio::spawn(async move {
        // send message and wait data
        // write message
        match writer.send("tests".into()).await {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error caused when writing stream")
            }
        }
    });

    // create thread to manage reading message
    let read_thread = tokio::spawn(async move {
        // pending flush buffer and read message if possible.
        let message = reader.next().await.unwrap().unwrap();
        match tokio::io::stdout()
            .write_all(format!("recv[]: {}\n", message).as_bytes())
            .await
        {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error caused when reading stream")
            }
        }
    });
    // run all threads.
    let res = future::try_join(write_thread, read_thread).await;
    match res {
        Ok((_, _)) => {
            println!("successfully finished");
        }
        Err(e) => {
            eprintln!("Error in thread=>\n{}", e);
        }
    }

    return Ok(());
}
