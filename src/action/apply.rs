use crate::peer_conf::PeerConfig;

pub fn apply() {
    println!("Applying configs");
    PeerConfig::reload().unwrap();
    println!("Config applied");
}
