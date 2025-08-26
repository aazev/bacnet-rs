use bacnet::application::*;
use bacnet::network::*;
use bacnet::transport::bacnetip::*;
use bacnet::{Decode, Encode};

use async_std::net::UdpSocket;
use async_std::task;

fn main() {
    tracing_subscriber::fmt::init();

    task::block_on(async {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", 0)).await.unwrap();
        socket.set_broadcast(true).unwrap();
        let mut buf = vec![0u8; 262144];

        println!("Listening on {}", socket.local_addr().unwrap());

        //let addr = format!("192.168.69.255:{}", 0xBAC0);
        // let addr = format!("172.17.0.2:{}", 0xBAC0);
        // let addr = format!("192.168.15.10:{}", 0xBAC0);
        // let addr = format!("172.30.11.118:{}", 0xBAC0);
        // let addr = format!("172.30.10.149:{}", 0xBAC0);
        let addr = format!("172.30.10.130:{}", 0xBAC0);
        // let addr = format!("172.30.10.177:{}", 0xBAC0);
        let data_ref = hex::decode("810b000c0120ffff00ff1008").unwrap(); // Who-is
        let apdu = APDU::new(0x01, 0x08, vec![]);
        println!("APDU Len: {}", apdu.len());
        let dest = NPDUDest::new(0xffff, 0);
        let npdu = NPDU::new(apdu, Some(dest), None, NPDUPriority::Normal);
        let bvlc = BVLC::new(BVLCFunction::OriginalBroadcastNPDU(npdu));
        let data = bvlc.encode_vec().unwrap();
        println!("Who-Is: {:?}", bvlc);
        println!("Send: {:02x?}", data.to_vec());
        println!("Ref : {:02x?}", data_ref);
        let sent = socket.send_to(&data, &addr).await.unwrap();
        println!("Sent {} bytes to {}", sent, addr);

        loop {
            let (n, peer) = socket.recv_from(&mut buf).await.unwrap();
            buf = buf[..n].to_vec();
            println!("Received {} bytes from {}", n, peer);
            //let buf = buf[5..buf[4] as usize + 5].to_vec();
            // === Data Structure ===
            println!("Data: {:02x?}", buf);

            let b = BVLC::decode_slice(&buf).unwrap();
            println!("BVLC: {:02x?}", b);
            println!("Function: {:02x?}", b.function);
            println!("Length: {:?}", b.len());

            match b.function {
                BVLCFunction::OriginalBroadcastNPDU(n) | BVLCFunction::OriginalUnicastNPDU(n) => {
                    println!("NPDU: {:02x?}", n);
                    println!("Version: {}", n.version);
                    println!("Priority: {:?}", n.priority);
                    match n.content {
                        NPDUContent::APDU(apdu) => {
                            println!("APDU: {:02x?}", apdu);
                            match apdu.service_choice {
                                8 => {
                                    println!("Who-Is received!");
                                    //let apdu = APDU::new();
                                    //let sent = socket.send_to().await.unwrap();
                                }
                                0 => {
                                    println!("I-Am received!");
                                }
                                _ => println!("Unknown Service Choice: {}", apdu.service_choice),
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
            }
        }
    });
}
