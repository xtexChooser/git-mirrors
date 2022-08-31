use super::Tunnel;

pub trait TunnelManager {
    fn get_tunnels() -> Vec<Box<dyn Tunnel>>;
}
