use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

mod computer;
mod permuter;

use computer::{Computer, IO};
use permuter::Permuter;

fn main() {
    let mut maximum = 0;

    for v in Permuter::new() {
        let value = test_feedback_loop(&v[..]);
        if value > maximum {
            maximum = value;
        }
    }

    println!("Maximum Output: {}", maximum);
}

fn test_feedback_loop(phases: &[i32]) -> i32 {
    let (sender_e, receiver_a) = channel();
    let (sender_a, receiver_b) = channel();
    let (sender_b, receiver_c) = channel();
    let (sender_c, receiver_d) = channel();
    let (sender_d, receiver_e) = channel();

    let mut iterator = phases.iter();
    sender_e.send(*iterator.next().unwrap()).unwrap();
    sender_e.send(0).unwrap();
    sender_a.send(*iterator.next().unwrap()).unwrap();
    sender_b.send(*iterator.next().unwrap()).unwrap();
    sender_c.send(*iterator.next().unwrap()).unwrap();
    sender_d.send(*iterator.next().unwrap()).unwrap();

    let handle_a = run_amplifier(AmplifierIO::new(sender_a, receiver_a));
    let handle_b = run_amplifier(AmplifierIO::new(sender_b, receiver_b));
    let handle_c = run_amplifier(AmplifierIO::new(sender_c, receiver_c));
    let handle_d = run_amplifier(AmplifierIO::new(sender_d, receiver_d));
    let handle_e = run_amplifier(AmplifierIO::new(sender_e, receiver_e));

    let io_a = handle_a.join().unwrap();
    handle_b.join().unwrap();
    handle_c.join().unwrap();
    handle_d.join().unwrap();
    handle_e.join().unwrap();

    io_a.receiver.recv().unwrap()
}

fn run_amplifier(mut io: AmplifierIO) -> thread::JoinHandle<AmplifierIO> {
    thread::spawn(move || {
        let computer = Computer::new();
        computer.run(&mut io);
        io
    })
}

struct AmplifierIO {
    sender: Sender<i32>,
    receiver: Receiver<i32>,
}

impl AmplifierIO {
    fn new(sender: Sender<i32>, receiver: Receiver<i32>) -> Self {
        AmplifierIO { sender, receiver }
    }
}

impl IO for AmplifierIO {
    fn next_input(&mut self) -> i32 {
        self.receiver.recv().unwrap()
    }

    fn next_output(&mut self, value: i32) {
        self.sender.send(value).unwrap()
    }
}
