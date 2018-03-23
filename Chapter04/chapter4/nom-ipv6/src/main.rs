#[macro_use]
extern crate nom;

use std::net::Ipv6Addr;

use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct IPv6Header {
    version: u8,
    traffic_class: u8,
    flow_label: u32,
    payload_length: u16,
    next_header: u8,
    hop_limit: u8,
    source_addr: Ipv6Addr,
    dest_addr: Ipv6Addr,
}

fn slice_to_array(input: &[u8]) -> [u8; 16] {
    let mut array = [0u8; 16];
    for (&x, p) in input.iter().zip(array.iter_mut()) {
        *p = x;
    }
    array
}

fn to_ipv6_address(i: &[u8]) -> Ipv6Addr {
    let arr = slice_to_array(i);
    Ipv6Addr::from(arr)
}

named!(parse_version<&[u8], u8>, bits!(take_bits!(u8, 4)));
named!(parse_traffic_class<&[u8], u8>, bits!(take_bits!(u8, 8)));
named!(parse_flow_label<&[u8], u32>, bits!(take_bits!(u32, 20)));
named!(parse_payload_length<&[u8], u16>, bits!(take_bits!(u16, 16)));
named!(parse_next_header<&[u8], u8>, bits!(take_bits!(u8, 8)));
named!(parse_hop_limit<&[u8], u8>, bits!(take_bits!(u8, 8)));
named!(parse_address<&[u8], Ipv6Addr>, map!(take!(16), to_ipv6_address));

named!(ipparse<&[u8], IPv6Header>,
       do_parse!(
            ver: parse_version >>
            cls: parse_traffic_class >>
            lbl: parse_flow_label >>
            len: parse_payload_length >>
            hdr: parse_next_header >>
            lim: parse_hop_limit >>
            src: parse_address >>
            dst: parse_address >>
              (IPv6Header {
                  version: ver,
                  traffic_class: cls,
                  flow_label: lbl,
                  payload_length: len,
                  next_header: hdr,
                  hop_limit: lim,
                  source_addr: src,
                  dest_addr : dst
              })
));

pub fn parse_ipv6_header(i: &[u8]) -> IResult<&[u8], IPv6Header> {
    ipparse(i)
}

fn main() {
    const EMPTY_SLICE: &'static [u8] = &[];
    let bytes = [0x60,
                 0x00,
                 0x08, 0x19,
                 0x80, 0x00, 0x14, 0x06,
                 0x40,
                 0x2a, 0x02, 0x0c, 0x7d, 0x2e, 0x5d, 0x5d, 0x00, 0x24, 0xec, 0x4d, 0xd1, 0xc8, 0xdf, 0xbe, 0x75,
                 0x2a, 0x00, 0x14, 0x50, 0x40, 0x0c, 0x0c, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbd
                 ];

    let expected = IPv6Header {
        version: 6,
        traffic_class: 0,
        flow_label: 33176,
        payload_length: 20,
        next_header: 6,
        hop_limit: 64,
        source_addr: "2a02:c7d:2e5d:5d00:24ec:4dd1:c8df:be75".parse().unwrap(),
        dest_addr: "2a00:1450:400c:c0b::bd".parse().unwrap(),
    };
    assert_eq!(ipparse(&bytes), IResult::Done(EMPTY_SLICE, expected));
}
