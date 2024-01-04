use core::time;
use std::{env, thread};

use mini_msg::core::node::Node;
use mini_msg::transporter::redis::RedisTransporter;

fn main() {
  let args = env::args().collect::<Vec<String>>();
  let this_node_name = args.get(1).expect("Node name to be provided");
  let that_node_name = args.get(2).expect("Node name to be provided");
  let node = Node::new(Some(this_node_name));

  let mut transporter = RedisTransporter::new("localhost", 6379);
  loop {
    node.send(
      &mut transporter,
      that_node_name,
      &format!("Hello, from {}!", node.id()),
    );
    println!("RECEIVED MESSAGE: {:#?}", node.read(&mut transporter));

    thread::sleep(time::Duration::from_millis(500));
  };
}
