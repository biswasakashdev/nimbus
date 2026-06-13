mod byte_processor_grpc;

use byte_processor_grpc::handler;

fn main() {
    let str = handler();
    println!("{str}");
}
