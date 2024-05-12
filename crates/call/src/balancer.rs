use crate::{endpoint::ReplicaConnection, options::CallOptions};

// Balancer manages a set of ReplicaConnections and picks one of them per
// call. It requires external synchronization (no concurrent calls).
pub trait Balancer {
    // Add adds a ReplicaConnection to the set of connections.
    fn add(&mut self, conn: Box<dyn ReplicaConnection + Sync + Send + 'static>);

    // Remove removes a ReplicaConnection from the set of connections.
    fn remove(&mut self, conn: &dyn ReplicaConnection);

    // Pick picks a ReplicaConnection from the set of connections based on
    // provided CallOptions. Returns None if no connections are available.
    fn pick(&mut self, options: &CallOptions) -> Option<Box<dyn ReplicaConnection>>;
}

// RoundRobin implements Balancer with a round-robin picking strategy.
struct RoundRobin {
    connections: Vec<Box<dyn ReplicaConnection>>,
    next: usize,
}

impl Balancer for RoundRobin {
    fn add(&mut self, conn: Box<dyn ReplicaConnection + Sync + Send + 'static>) {
        self.connections.push(conn)
    }

    fn remove(&mut self, conn: &dyn ReplicaConnection) {
        let index_to_remove = self
            .connections
            .iter()
            .position(|x| x.address() == conn.address());
        if let Some(index) = index_to_remove {
            self.connections.remove(index);
        }
    }

    fn pick(&mut self, _options: &CallOptions) -> Option<Box<dyn ReplicaConnection>> {
        if self.connections.len() == 0 {
            return None;
        }
        if self.next >= self.connections.len() {
            self.next = 0;
        }
        let conn = self.connections.remove(self.next);
        self.next += 1;
        Some(conn)
    }
}

// RoundRobin returns a new RoundRobin balancer.
pub fn round_robin() -> Box<dyn Balancer> {
    Box::new(RoundRobin {
        connections: Vec::new(),
        next: 0,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        endpoint::{NetEndpoint, ReplicaConnection},
        options::CallOptions,
    };

    use super::round_robin;

    #[test]
    fn test_round_robin() {
        let mut rr = round_robin();

        let e1 = NetEndpoint::new("tcp", "127.0.0.1");
        let e2 = NetEndpoint::new("tcp", "127.0.0.2");

        rr.add(Box::new(e1.clone()));
        rr.add(Box::new(e2.clone()));

        let picked = rr.pick(&CallOptions {});
        assert!(picked.is_some());
        let e = picked.unwrap();
        assert_eq!(e.address(), e1.address());

        rr.remove(&e1);
        let picked = rr.pick(&CallOptions {});
        assert!(picked.is_some());
        let e = picked.unwrap();
        assert_eq!(e.address(), e2.address());
    }
}
