use minimsg::core::node::Node;
use minimsg::transporter::generic::GenericTransporter;

fn main() {
  let mut transporter = GenericTransporter::new();

  let node_0 = Node::new(Some("NODE_0"));
  let node_1 = Node::new(Some("NODE_1"));

  println!("NODE_1 RECEIVED MESSAGE: {:#?}", node_1.read(&mut transporter));
  node_0.send(&mut transporter, node_1.id(), "Hello, world!");
  println!("NODE_1 RECEIVED MESSAGE: {:#?}", node_1.read(&mut transporter));
  println!("NODE_1 RECEIVED MESSAGE: {:#?}", node_1.read(&mut transporter));
  println!("NODE_1 RECEIVED MESSAGE: {:#?}", node_1.read(&mut transporter));
}
