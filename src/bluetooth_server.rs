use std::sync::mpsc::Sender;
use std::time::Duration;

use bluer::l2cap::Stream;
use bluer::{
    adv::Advertisement,
    l2cap::{SocketAddr, StreamListener},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout_at, Instant};

use crate::GameCommand;
const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xFEED0000F00D);

include!("l2cap.inc");

pub async fn run_bluetooth_server(rx: Sender<GameCommand>) -> bluer::Result<()> {
    // activate bluethooth "boilerplate"
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    let adapter_addr = adapter.address().await?;
    let adapter_addr_type = adapter.address_type().await?;

    println!(
        "Advertising on Bluetooth adapter {} with {} address {}",
        adapter.name(),
        &adapter_addr_type,
        &adapter_addr
    );

    let le_advertisement = Advertisement {
        service_uuids: vec![SERVICE_UUID].into_iter().collect(),
        discoverable: Some(true),
        local_name: Some("l2cap_server".to_string()),
        ..Default::default()
    };

    let _adv_handle = adapter.advertise(le_advertisement).await?;
    let local_sa = SocketAddr::new(adapter_addr, adapter_addr_type, PSM);
    let listener = StreamListener::bind(local_sa).await?;
    println!("Listening on PSM {}.", listener.as_ref().local_addr()?.psm);
    let mut next_id = 1;

    // wait for connections and start a task for each
    loop {
        // this part is a bit weird:
        // we await `listener.accept` and if it's not an error, put everything into the variables
        // we should just be able to:
        //let (mut stream, sa) = listener.accept().await.unwrap_or(continue);
        // this does not seam to work though - no idea why
        // this does though
        let (mut stream, sa) = tokio::select! {
            l = listener.accept() => {
                match l {
                    Ok(v) => v,
                    Err(err) => {
                        println!("Accepting connection failed: {}", &err);
                        continue;
                    }
                }
            }
        };
        let recv_mtu = stream.as_ref().recv_mtu()?;

        println!(
            "Accepted connection from {:?} with receive MTU {} bytes",
            &sa, &recv_mtu
        );

        let addr = sa.addr.to_string();
        if let Err(e) = stream.write_all(&[next_id]).await {
            println!("write for {addr} failed!: \"{e}\"");
        }
        tokio::spawn(bluetooth_loop(stream, addr, next_id, rx.clone()));
        next_id = next_id.wrapping_add(1);
    }
}

/// address is just used for display purpose - don't need the real one
async fn bluetooth_loop(
    mut stream: Stream,
    address: String,
    id: u8,
    tx: Sender<GameCommand>,
) -> bluer::Result<()> {
    tx.send(GameCommand::Join { id })
        .expect(&format!("{id} failed to join"));
    let mut should_exit = false;
    while !should_exit {
        let mut buf: [u8; 4] = [0; 4];
        let poll = timeout_at(Instant::now() + Duration::from_secs(60), async {
            if let Err(e) = stream.read(&mut buf).await {
                println!("got error: \"{e}\" - disconnecting");
                tx.send(GameCommand::Leave { id })
                    .expect(&format!("{id}failed to Leave"));
                should_exit = true;
                return;
            }
            if buf[0] == 0 {
                // has joined - maybe do something here
            } else {
                tx.send(GameCommand::Strike {
                    id,
                    strike: [buf[1].clamp(0, 2), buf[2], buf[3]],
                })
                .expect(&format!("{id}failed to Send"));
            }
            println!("strike from {id}: {:#?}", buf);
        });

        if let Err(_) = poll.await {
            println!("timeout reached for device {}. disconnecting...", address);
            tx.send(GameCommand::Leave { id })
                .expect("could not disconnect from game");
            should_exit = true;
        }
    }
    Ok(())
}
