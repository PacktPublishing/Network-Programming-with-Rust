extern crate grpc_example;
extern crate grpc;

use grpc_example::foobar::*;
use grpc_example::foobar_grpc::*;


fn main() {
    let client = FooBarServiceClient::new_plain("127.0.0.1", 9001, Default::default()).unwrap();

    let mut req = CabLocationRequest::new();
    req.set_name("foo".to_string());

    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    req.set_location(location);

    let resp = client.record_cab_location(grpc::RequestOptions::new(), req);
    match resp.wait() {
        Err(e) => panic!("{:?}", e),
        Ok((_, r, _)) => println!("{:?}", r),
    }

    let mut nearby_req = GetCabRequest::new();
    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;    
    nearby_req.set_location(location);

    let nearby_resp = client.get_cabs(grpc::RequestOptions::new(), nearby_req);
    match nearby_resp.wait() {
        Err(e) => panic!("{:?}", e),
        Ok((_, cabs, _)) => println!("{:?}", cabs),
    }
}
