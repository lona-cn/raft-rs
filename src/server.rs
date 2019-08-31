//this to found set_boxed_logger
mod core;
mod protos_gen;


use futures::future::Future;
use futures::sync::oneshot;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use protos_gen::diner::{Check, Item, Order};
use protos_gen::diner_grpc::{self, Diner};
use std::io;
use std::io::Read;
use std::sync::Arc;
use std::thread;

#[cfg(all(feature = "std", atomic_cas))]
mod tests;
#[derive(Clone)]
struct DinerService;

impl Diner for DinerService {
    fn eat(&mut self, ctx: RpcContext, order: Order, sink: UnarySink<Check>) {
        println!("Received Order {{ {:?} }}", order);
        let mut check = Check::new();
        check.set_total(order.get_items().iter().fold(0.0, |total, &item| {
            total
                + match item {
                Item::Spam => 0.05,
                Item::Eggs => 0.25,
                Item::Ham => 1.0,
            }
        }));

        let f = sink
            .success(check.clone())
            .map(move |_| println!("Reponsed with Check {{ {:?} }}", check))
            .map_err(move |err| eprintln!("Failed to reply:{:?}", err));
        ctx.spawn(f);
    }
}

fn main() {
    //Arc == Atomically Reference Counted
    let env = Arc::new(Environment::new(1));
    let service = diner_grpc::create_diner(DinerService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 54779)
        .build()
        .unwrap();
    server.start();

    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        println!("Press Enter to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
