use tmq::{subscribe, Result};
use zmq::{Context, SocketType};

use utils::{check_receive_multiparts, generate_tcp_address, sync_send_multipart_repeated};

mod utils;

#[tokio::test]
async fn receive_single_message() -> Result<()> {
    let address = generate_tcp_address();
    let ctx = Context::new();
    let topic: &[u8] = b"topic2";
    let sock = subscribe(&ctx).bind(&address)?.subscribe(topic)?;

    let data = vec![topic, b"hello", b"world"];

    // hack to send long enough for the subscriber to receive something
    sync_send_multipart_repeated(address, SocketType::PUB, data.clone(), 10000);

    check_receive_multiparts(sock, vec![data]).await?;

    Ok(())
}
