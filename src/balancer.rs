use rand::seq::SliceRandom;

pub fn roundrobin(addrlist: Vec<String>) -> String {
    addrlist
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}
