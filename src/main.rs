use ::futures::future;
use std::time::Duration;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::mpsc::unbounded_channel,
};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt()]
struct Opt {
    #[structopt(long, default_value = ";")]
    status_code_delimeter: String,

    #[structopt(long, default_value = "100")]
    sleep_milli: u64,
}

#[tokio::main]
async fn main() {
    // http url을 stdin으로 부터 읽어 들인다
    let opt = Opt::from_args();
    let mut buf_reader = BufReader::new(tokio::io::stdin());
    let client = reqwest::Client::new();
    let mut join_handles = vec![];

    let (tx, mut rx) = unbounded_channel::<(String, u16)>();

    tokio::spawn(async move {
        while let Some((url, status)) = rx.recv().await {
            println!("{}{}{}", url, opt.status_code_delimeter, status);
        }
    });

    loop {
        let mut url = String::new();
        let num_bytes = buf_reader.read_line(&mut url).await;

        if let Ok(num_bytes) = num_bytes {
            if num_bytes == 0 {
                break;
            }
        } else {
            break;
        }

        if url.starts_with("http") == false {
            break;
        }

        let client_ref = client.clone();
        let tx_ref = tx.clone();

        join_handles.push(tokio::spawn(async move {
            let resp = client_ref.get(&url).send().await.unwrap();
            let status = resp.status().as_u16();

            tx_ref.send((url.trim().to_owned(), status)).unwrap();
        }));

        tokio::time::sleep(Duration::from_millis(opt.sleep_milli)).await;
    }

    future::join_all(join_handles.iter_mut()).await;
}
