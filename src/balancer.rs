use crate::config::ServiceConfig;
use rand::seq::SliceRandom;

// refactor to add init function that pre-computes stuff and hands it off to functions, e.g. roundrobin

pub fn handle(method: &str, addrlist: Vec<ServiceConfig>) -> String {
    match method {
        "roundrobin" => roundrobin(addrlist),
        _ => roundrobin(addrlist),
    }
}

pub fn roundrobin(addrlist: Vec<ServiceConfig>) -> String {
    addrlist
        .choose(&mut rand::thread_rng())
        .unwrap()
        .address
        .to_string()
}
