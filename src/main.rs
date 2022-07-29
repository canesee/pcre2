#![allow(dead_code, non_camel_case_types)]

mod bindings;
mod ffi;
mod regex;

use futures::stream::{self, StreamExt};
use regex::Regex;
use std::net::UdpSocket;
use std::sync::Arc;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "pcre2")]
struct Opt {
    #[structopt(short, long, default_value = "127.0.0.1:34254")]
    local: String,
    #[structopt(short, long, default_value = "127.0.0.1:16523")]
    peer: String,
}

#[tokio::main]
async fn main() {
    let args = Opt::from_args();
    let local = Arc::new(UdpSocket::bind(args.local).expect("could not bind to addresss"));
    let peer = Arc::new(args.peer);

    let pattern = r"\d\d\d\d[^0-9\s]{3,11}[\S]";
    let subject = b"a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopiqa=)*(^!@#$%^&*())9999999";
    let regex = Regex::new(pattern).expect("regex new error");

    let ovector = regex.find_iter(subject);
    stream::iter(ovector)
        .for_each_concurrent(1024, |data| {
            let local = Arc::clone(&local);
            let peer = Arc::clone(&peer);
            async move {
                if let Ok(data) = data {
                    local
                        .send_to(&subject[data.start() + 4..data.end() - 1], &*peer)
                        .expect("could not send data");
                } else {
                    println!("data error");
                }
            }
        })
        .await;
}
