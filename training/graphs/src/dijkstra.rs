use std::{collections::BTreeSet, ops::Add};

#[derive(Debug)]
pub struct WeightedGraph {
    edges: Vec<Vec<(usize, u64)>>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Distance {
    Reachable(u64),
    Infinity,
}

impl Add<u64> for Distance {
    type Output = Self;

    fn add(self, b: u64) -> Self::Output {
        match self {
            Self::Reachable(a) => Self::Reachable(a + b),
            Self::Infinity => Self::Infinity,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct EstimateDistance {
    distance: Distance,
    node: usize,
}

fn pop_first<K: Ord + Copy>(set: &mut BTreeSet<K>) -> K {
    let first = *set.iter().next().unwrap();
    set.take(&first).unwrap()
}

impl WeightedGraph {
    pub fn dijkstra(&self, start: usize) -> Vec<Distance> {
        assert!(start < self.edges.len());

        // Establish array with distances from `start` to the given node.
        let mut distances = vec![Distance::Infinity; self.edges.len()];
        distances[start] = Distance::Reachable(0);

        // Establish queue with the minimum computed distance to the given node.
        let mut queue = BTreeSet::new();
        queue.insert(EstimateDistance {
            node: start,
            distance: distances[start],
        });

        while !queue.is_empty() {
            // Take node with the minimum distance.
            let node = pop_first(&mut queue).node;

            for &(to, len) in &self.edges[node] {
                // Compute distance between the taken node and one of the edge nodes.
                let new_distance = distances[node] + len;
                // If this distance is less than previously calculated perform distance's "relax".
                if new_distance < distances[to] {
                    // Remove old distance value from the minimal distances queue.
                    queue.remove(&EstimateDistance {
                        node: to,
                        distance: distances[to],
                    });

                    // Update distance from `start` for the `to` node.
                    distances[to] = new_distance;
                    // Insert a new distance to the queue.
                    queue.insert(EstimateDistance {
                        node: to,
                        distance: new_distance,
                    });
                }
            }
        }

        distances
    }
}

#[test]
fn distance_ord() {
    assert!(Distance::Reachable(0) < Distance::Infinity);
}

#[test]
fn estimate_distance_ord() {
    assert!(
        EstimateDistance {
            node: 2,
            distance: Distance::Reachable(10)
        } < EstimateDistance {
            node: 0,
            distance: Distance::Reachable(20)
        }
    )
}

#[test]
fn dijkstra() {
    let graph = WeightedGraph {
        edges: vec![
            vec![(1, 7), (2, 9), (5, 14)],
            vec![(2, 10), (3, 15)],
            vec![(3, 11), (5, 2)],
            vec![],
            vec![(3, 6)],
            vec![(4, 9)],
        ],
    };

    let distances = graph.dijkstra(0);

    assert_eq!(
        distances,
        vec![
            Distance::Reachable(0),
            Distance::Reachable(7),
            Distance::Reachable(9),
            Distance::Reachable(20),
            Distance::Reachable(20),
            Distance::Reachable(11),
        ]
    );
}
