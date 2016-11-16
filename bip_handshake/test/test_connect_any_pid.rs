use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use bip_handshake::{BTHandshaker, Handshaker};

use {MockContext, MockProtocol, MockEvent};

#[test]
fn positive_connect_any_pid() {
    // Create dummy metadata channels
    let (m_send, _): (Sender<()>, Receiver<()>) = mpsc::channel();

    // Create a context that both protocols can access
    let (context, recv) = MockContext::new();

    // Store peer ids and the info hash
    let pid_one = [0u8; 20].into();
    let pid_two = [1u8; 20].into();
    let info_hash = [0u8; 20].into();

    // Create two handshakers to connect to each other
    let mut handshaker_one = BTHandshaker::new::<MockProtocol>(m_send.clone(), "127.0.0.1:0".parse().unwrap(), pid_one, context.clone()).unwrap();
    let handshaker_two = BTHandshaker::new::<MockProtocol>(m_send, "127.0.0.1:0".parse().unwrap(), pid_two, context).unwrap();

    // Make sure both handshakers are looking for the same info hash
    handshaker_one.register(info_hash);
    handshaker_two.register(info_hash);

    // Have handshaker one initiate a connection with handshaker two but we dont know the PeerId
    handshaker_one.connect(None, info_hash, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), handshaker_two.port())));

    // Allow the handshakers time to complete
    thread::sleep(Duration::from_millis(250));

    // Assert that both handshakers sent their message
    assert_eq!(recv.try_recv(), Ok(MockEvent::Connect));
    assert_eq!(recv.try_recv(), Ok(MockEvent::Connect));
    assert!(recv.try_recv().is_err());
}