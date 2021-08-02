#[warn(non_snake_case)]
mod udp_client;

use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::*;
use std::sync::{mpsc, Arc, Mutex};
use std::time::SystemTime;
use std::{thread, time};

type ConType = Arc<Mutex<Vec<udp_client::Client>>>;

// after this time inactive clients will be removed.
// seconds
const INACTIVE_TIME: u64 = 30;

fn main() {
    println!("Server Started!");

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 55442));
    let socket: UdpSocket = UdpSocket::bind(addr).expect("couldn't bind to address");

    // let mut connections: Vec<Client> = Vec::new();
    // let mut connections: Mutex<Vec<Client>> = Mutex::new(Vec::new());
    let connections: ConType = Arc::new(Mutex::new(Vec::new()));
    //let asdf = Arc::clone(&connections);
    //let mut connections:Rc<Vec<Client>> = Rc::new(Vec::new());
    let (sender, receiver): (Sender<Sms>, Receiver<Sms>) = mpsc::channel();
    // let (sender, receiver):(Sender<Vec<u8>>, Receiver<Vec<u8>>)= mpsc::channel();

    {
        let conn = Arc::clone(&connections);
        let sock = socket.try_clone();
        thread::spawn(move || {
            listener(&sock.unwrap(), sender, conn); //&mut Rc::borrow_mut(&mut connections)
        });
    }

    {
        let conn = Arc::clone(&connections);
        thread::spawn(move || loop {
            cleaner(&conn);
            thread::sleep(time::Duration::from_secs(1));
        });
    }

    postmen(&socket, receiver, Arc::clone(&connections));
}

fn listener(socket: &UdpSocket, sender: mpsc::Sender<Sms>, conns: ConType) {
    //socket: &UdpSocket, sender:mpsc::Sender<Vec<u8>>, conns: ConType
    // sender:mpsc::Sender<Vec<u8>>
    // socket_addr: &SocketAddr
    // let socket:UdpSocket = UdpSocket::bind(socket_addr).expect("couldn't bind to address");
    // let next = socket.try_clone();
    //&mut Vec<Client> //  Rc<Vec<Client>> &mut Mutex<Vec<Client>>
    let mut buffer: [u8; 1400] = [0; 1400];
    loop {
        let (len, addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");

        {
            let mut is_add = true;
            //conns.lock().unwrap();
            let mut connections = conns.lock().unwrap();
            for cl in connections.iter_mut() {
                // iter() conns.iter_mut().lock().unwrap()
                if cl.addr.eq(&addr) {
                    is_add = false;
                    cl.date_of_last_message = SystemTime::now();
                }
            }

            if is_add {
                let adr = addr.clone();

                let cl = udp_client::new(adr, 0);
                //Client {
                //     addr: adr,
                //     date_of_last_message: SystemTime::now(),
                //     status: true,
                //     id: 0
                // };
                println!("add new client with addr: {:#?}", addr.clone());
                connections.push(cl);
            }
        }

        let message_to_channel = Sms {
            addr,
            data: buffer[..len].to_vec(),
        };

        sender
            .send(message_to_channel)
            .expect("couldn't send to channel");
        // sender.send(buffer[..len].to_vec()).expect("couldn't send to address");
    }
}

fn postmen(socket: &UdpSocket, reader: mpsc::Receiver<Sms>, conns: ConType) {
    // socket: &UdpSocket
    // reader:mpsc::Receiver<Vec<u8>>
    // socket_addr: &SocketAddr
    // let adr = socket_addr.clone();
    // let socket:UdpSocket = UdpSocket::bind(adr).expect("couldn't bind to address");
    // &Vec<Client> &Mutex<Vec<Client>>
    //     loop {
    //         {
    for message in reader {
        // let addrs = conns.lock().unwrap().as_slice().iter();
        for cl in conns.lock().unwrap().iter() {
            // .as_slice() // conns.lock().unwrap().as_slice().iter()
            // conns.lock().iter()
            if message.addr.ne(&cl.addr) && cl.status.ne(&false) {
                socket.send_to(&message.data.as_slice(), cl.addr).unwrap();
            }
        }
    }
    //     }
    // }
}

fn cleaner(conns: &ConType) {
    // socket: &UdpSocket
    let mut cons = conns.lock().unwrap(); // .to_vec(); // as_slice()

    let mut remove_conn: Vec<usize> = Vec::new();

    for (index, cl) in cons.iter_mut().enumerate() {
        if SystemTime::now()
            .duration_since(cl.date_of_last_message)
            .unwrap()
            .as_secs()
            > INACTIVE_TIME
        {
            cl.status = false;
            remove_conn.push(index);
        }
    }

    for index in remove_conn.iter() {
        if cons[*index].status.eq(&false) {
            let adr = cons[*index].addr.clone();
            cons.remove(*index);
            println!("removed client by {:#?}", adr);
        } //get(index).unwrap()
    }
}

// #[derive(Copy, Clone)]
// pub struct Player {
//     pub position: Vec3,
//     pub id: i32,
//     pub status: bool,
//     pub heals: u32,
// }
//
// type Vec3 = (f32, f32, f32);

pub struct Sms {
    pub addr: SocketAddr,
    pub data: Vec<u8>,
}
