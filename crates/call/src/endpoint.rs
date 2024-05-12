// ReplicaConnection is a trait representing a connection to a replica.
pub trait ReplicaConnection {
    // Address returns the name of the endpoint to which the ReplicaConnection
    // is connected.
    fn address(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct NetEndpoint {
    pub net: String,
    pub addr: String,
}

impl NetEndpoint {
    pub fn new(net: &str, addr: &str) -> Self {
        Self {
            net: net.to_owned(),
            addr: addr.to_owned(),
        }
    }
}

impl ReplicaConnection for NetEndpoint {
    fn address(&self) -> String {
        format!("{}://{}", self.net, self.addr)
    }
}
