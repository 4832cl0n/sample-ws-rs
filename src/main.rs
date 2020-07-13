extern crate ws;

use ws::{
    Sender,
    Handshake,
    Handler,
    Message,
    CloseCode,
    listen,
};
use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;

#[derive(Clone, Debug)]
struct Client {
    out: Sender,
    user_id: u32,
    client_list: Rc<RefCell<Vec<Client>>>,
}

impl Handler for Client {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        self.client_list.borrow_mut().iter().filter(|user| user.user_id != self.user_id)
        .for_each(|user|{
             user.out.send(msg.clone());
        });
        Ok(())
    }

    fn on_open<'a>(&'a mut self, _: Handshake) -> ws::Result<()> {
      print!("current user is below.");
      Ok(self.client_list.borrow_mut().iter().for_each(|client|{
          println!("{:?}", client.user_id);
      }))
    }
}

fn main() {
  let client_list = Rc::new(RefCell::new(vec!()));

  listen("0.0.0.0:9000", move |out| { 
    let client = Client { 
      out: out.clone(),
      user_id: rand::thread_rng().gen(),
      client_list: client_list.clone(),
    };
    client_list.borrow_mut().push(client.clone());

    client
  }).unwrap()
} 
