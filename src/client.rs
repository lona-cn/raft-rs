mod protos_gen;

use grpcio::{ChannelBuilder, EnvBuilder, Error};
use protos_gen::diner::{Check, Item, Order};
use protos_gen::diner_grpc::DinerClient;
use std::env;
use std::sync::Arc;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Expected exactly one argument, the port to connect to.")
    }

    let port = args[1]
        .parse::<u16>()
        .unwrap_or_else(|_| panic!("{} is not a valid port number", args[1]));

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", port).as_str());
    let client = DinerClient::new(ch);
    let mut order = Order::new();
    order.set_items(vec![Item::Spam, Item::Eggs]);
    let check = client.eat(&order).expect("RPC failed!");
    println!("Ate {:?} and got charged ${:.2}", order, check.get_total());
}
