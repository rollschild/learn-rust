#![allow(unused_variables)]

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug, Clone, Copy)]
struct CubeSat {
    id: u64,
}

#[derive(Debug, Clone, Copy)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }
}

impl CubeSat {
    fn receive(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![0, 1, 2, 3]
}

fn main() {
    let base = GroundStation {};
    let mut mail = Mailbox { messages: vec![] };
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        let msg = Message {
            to: sat_id,
            content: String::from("Hey, how are you?"),
        };
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        let msg = sat.receive(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}
