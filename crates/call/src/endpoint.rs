// ReplicaConnection is a trait representing a connection to a replica.
pub trait Endpoint: Sync + Send + 'static {
    fn clone_box(&self) -> Box<dyn Endpoint>;

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

impl Default for NetEndpoint {
    fn default() -> Self {
        Self {
            net: Default::default(),
            addr: Default::default(),
        }
    }
}

impl Endpoint for NetEndpoint {
    fn address(&self) -> String {
        format!("{}://{}", self.net, self.addr)
    }

    fn clone_box(&self) -> Box<dyn Endpoint> {
        return Box::new(NetEndpoint::clone(&self));
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoint::Endpoint;

    use super::NetEndpoint;

    #[test]
    fn test_endpoint() {
        let e = NetEndpoint::new("tcp", "127.0.0.1");
        assert!(e.address() == "tcp://127.0.0.1")
    }
}
