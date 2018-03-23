extern crate grpc_example;
extern crate grpc;
extern crate protobuf;

use std::thread;

use grpc_example::foobar_grpc::*;
use grpc_example::foobar::*;

struct FooBarServer;

impl FooBarService for FooBarServer {
    fn record_cab_location(&self,
                       _m: grpc::RequestOptions,
                       req: CabLocationRequest)
                       -> grpc::SingleResponse<CabLocationResponse> {
        let mut r = CabLocationResponse::new();

        println!("Recorded cab {} at {}, {}", req.get_name(), req.get_location().latitude, req.get_location().longitude);

        r.set_accepted(true);
        grpc::SingleResponse::completed(r)
    }

    fn get_cabs(&self,
                      _m: grpc::RequestOptions,
                      _req: GetCabRequest)
                      -> grpc::SingleResponse<GetCabResponse> {
        let mut r = GetCabResponse::new();

        let mut location = Location::new();
        location.latitude = 40.7128;
        location.longitude = -74.0060;

        let mut one = Cab::new();
        one.set_name("Limo".to_owned());
        one.set_location(location.clone());

        let mut two = Cab::new();
        two.set_name("Merc".to_owned());
        two.set_location(location.clone());

        let vec = vec![one, two];
        let cabs = ::protobuf::RepeatedField::from_vec(vec);

        r.set_cabs(cabs);

        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(9001);
    server.add_service(FooBarServiceServer::new_service_def(FooBarServer));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("Could not start server");
    loop {
        thread::park();
    }
}
