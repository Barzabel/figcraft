//! A host chooses the numbers in its welcome (#309, finding 1): a sequence number or a last batch
//! id at the top of `u64` is refused or held there, never overflowed.

use crate::message::{Accepted, ClientMsg, Processed, ServerMsg};
use crate::{Client, NoRecompute, Outgoing, Server};
use figcraft_doc::Document;

/// A welcome from a real server, its sequence number and last batch id at `u64::MAX`.
fn hostile_welcome() -> ServerMsg {
    let mut server = Server::new(Document::new("Hostile"), "tok", Default::default());
    let ClientMsg::Hello(h) = Client::hello("f", "tok", "Ada", "#FF8800") else { panic!("not a hello") };
    let (_, out) = server.join(h).unwrap();
    let Some(Outgoing::To(_, mut welcome)) = out.into_iter().next() else { panic!("no welcome") };
    if let ServerMsg::Welcome { seq, processed, .. } = &mut welcome {
        *seq = u64::MAX;
        *processed = Processed { last_batch: u64::MAX, refused: Vec::new() };
    }
    welcome
}

fn renamed(c: &Client, name: &str) -> Document {
    let mut d = c.document().clone();
    let page = d.first_page().unwrap();
    d.node_mut(page).unwrap().name = name.into();
    d
}

#[test]
fn edits_after_a_welcome_at_the_last_batch_id_dont_overflow() {
    let mut c = Client::join(hostile_welcome()).unwrap();
    for name in ["One", "Two"] {
        let d = renamed(&c, name);
        assert!(c.edit(&d).is_ok(), "{name}");
    }
}

#[test]
fn changes_after_a_welcome_at_the_last_sequence_number_are_refused() {
    let mut c = Client::join(hostile_welcome()).unwrap();
    let msg = ServerMsg::Changes(Accepted { seq: 0, author: c.id() + 1, batch_id: 1, changes: Vec::new() });
    assert!(c.receive(msg, &NoRecompute).is_err(), "nothing follows the last sequence number: the client rejoins");
}
