use std::collections::VecDeque;
use std::thread;

use crossbeam::channel::{unbounded, Receiver, Sender};

mod computer;

use computer::{Computer, IO};

fn main() {
    let mut network = Network::new();

    for _ in 0..50 {
        network.add_element();
    }

    network.watch_messages();
}

#[derive(Copy, Clone)]
struct Packet {
    x: i64,
    y: i64,
}

struct Network {
    elements: Vec<NetworkElement>,
    packet_memory: Option<Packet>,
}

impl Network {
    fn new() -> Self {
        Network {
            elements: Vec::new(),
            packet_memory: None,
        }
    }

    fn add_element(&mut self) {
        let index = self.elements.len() as i64;
        let (message_sender, message_receiver) = unbounded();
        let (packet_sender, packet_receiver) = unbounded();
        self.elements.push(NetworkElement {
            sender: packet_sender,
            receiver: message_receiver,
        });

        thread::spawn(move || {
            let computer = Computer::new();
            let mut io = NicIO::new(index, message_sender, packet_receiver);

            computer.run(&mut io);
        });
    }

    fn watch_messages(&mut self) {
        let mut no_messages = false;
        loop {
            for element in self.elements.iter() {
                if let Ok(message) = element.receiver.try_recv() {
                    match message.destination {
                        255 => {
                            self.packet_memory = Some(message.packet);
                        }
                        a if 0 <= a && a < 50 => {
                            self.elements[a as usize]
                                .sender
                                .send(message.packet)
                                .unwrap();
                        }
                        _ => {}
                    }
                }
            }

            if self
                .elements
                .iter()
                .all(|e| e.sender.is_empty() && e.receiver.is_empty())
            {
                if no_messages {
                    if let Some(packet) = self.packet_memory {
                        self.elements[0].sender.send(packet).unwrap();
                        println!("Sent y value: {}", packet.y);
                    }
                    no_messages = false;
                } else {
                    no_messages = true;
                }
            }
        }
    }
}

struct NetworkElement {
    sender: Sender<Packet>,
    receiver: Receiver<Message>,
}

struct Message {
    destination: i64,
    packet: Packet,
}

struct NicIO {
    queue: VecDeque<i64>,
    network_sender: Sender<Message>,
    network_receiver: Receiver<Packet>,
    partial_message: VecDeque<i64>,
}

impl NicIO {
    fn new(address: i64, sender: Sender<Message>, receiver: Receiver<Packet>) -> Self {
        NicIO {
            queue: VecDeque::from(vec![address]),
            network_sender: sender,
            network_receiver: receiver,
            partial_message: VecDeque::new(),
        }
    }

    fn check_messages(&mut self) {
        if let Ok(packet) = self.network_receiver.try_recv() {
            self.queue.push_back(packet.x);
            self.queue.push_back(packet.y);
        }
    }
}

impl IO for NicIO {
    fn next_input(&mut self) -> i64 {
        self.check_messages();
        match self.queue.pop_front() {
            Some(val) => val,
            None => -1,
        }
    }

    fn next_output(&mut self, val: i64) {
        self.partial_message.push_back(val);
        if self.partial_message.len() == 3 {
            let message = Message {
                destination: self.partial_message.pop_front().unwrap(),
                packet: Packet {
                    x: self.partial_message.pop_front().unwrap(),
                    y: self.partial_message.pop_front().unwrap(),
                },
            };
            self.network_sender.send(message).unwrap();
        }
    }
}
