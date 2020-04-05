use std::collections::LinkedList;

#[derive(Debug)]
pub struct Graph {
    edges: Vec<Vec<usize>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Distance {
    Unreachable,
    Reachable(usize),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum NodeColor {
    /// Unvisited node,
    White,
    /// Currently visiting node.
    Gray,
    /// Visited node; all iterations with it are finished.
    Black,
}

impl Distance {
    fn increase(self) -> Self {
        match self {
            Self::Unreachable => unreachable!("DFS implementation is incorrect"),
            Self::Reachable(v) => Self::Reachable(v + 1),
        }
    }
}

impl Graph {
    pub fn bfs(&self, from: usize, to: usize) -> Distance {
        // Check that the start and end nodes exist.
        assert!(from < self.edges.len());
        assert!(to < self.edges.len());

        let mut distances = vec![Distance::Unreachable; self.edges.len()];
        // Next nodes to visit.
        let mut queue = LinkedList::new();
        let mut visited = vec![false; self.edges.len()];

        // Initialization: where is an information about the start node.
        queue.push_back(from);
        // Distance from the `from` node to the `from` node always is zero.
        distances[from] = Distance::Reachable(0);
        // Mark `from` node as visited.
        visited[from] = true;
        // While have nodes to visit perform `BFS`.
        while !queue.is_empty() {
            // Get first node from the queue
            let node = queue.pop_front().unwrap();
            // Go through all node neighbors
            for &neighbor in &self.edges[node] {
                // Check if neighbor is still `white`
                if !visited[neighbor] {
                    // Compute distance between start node and neighbor.
                    distances[neighbor] = distances[node].increase();
                    // Mark neighbor as `gray`.
                    visited[neighbor] = true;
                    // Add node to lookup queue.
                    queue.push_back(neighbor);
                }
            }
        }

        // Return distance to desired node.
        distances[to]
    }

    pub fn dfs(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        // Check that the start and end nodes exist.
        assert!(from < self.edges.len());
        assert!(to < self.edges.len());

        // Recursive 'dfs' implementation
        fn do_dfs(
            edges: &[Vec<usize>],
            visited: &mut [NodeColor],
            prior: &mut [Option<usize>],
            node: usize,
            from: usize,
            to: usize,
        ) {
            // If we have already visited this node, there is nothing to do.
            if visited[node] != NodeColor::White {
                return;
            }
            // Mark current node as `gray`.
            visited[node] = NodeColor::Gray;
            // Store node in the prior array.
            prior[node] = Some(from);

            // Go through all node neighbors.
            for &neighbor in &edges[node] {
                do_dfs(edges, visited, prior, neighbor, node, to);
            }
            // Mark current node as `black`
            visited[node] = NodeColor::Black;
        }

        // Visited nodes.
        let mut visited = vec![NodeColor::White; self.edges.len()];
        // Remember node from where did we come.
        let mut prior = vec![None; self.edges.len()];
        // Go through all `from` node neighbors.
        for &neighbor in &self.edges[from] {
            do_dfs(&self.edges, &mut visited, &mut prior, neighbor, from, to);
        }

        // Restore complete path `from` -> `to`
        let mut prev = prior[to]?;
        let mut path = vec![prev];
        while prev != from {
            prev = prior[prev]?;
            path.push(prev);
        }
        path.reverse();
        path.push(to);

        Some(path)
    }
}

#[test]
fn bfs_simple() {
    let graph = Graph {
        edges: vec![
            vec![1, 2, 3],
            vec![2, 0],
            vec![0, 1],
            vec![0, 4],
            vec![1, 3],
            vec![],
        ],
    };

    assert_eq!(graph.bfs(0, 0), Distance::Reachable(0));
    assert_eq!(graph.bfs(0, 2), Distance::Reachable(1));
    assert_eq!(graph.bfs(0, 4), Distance::Reachable(2));
    assert_eq!(graph.bfs(2, 3), Distance::Reachable(2));

    assert_eq!(graph.bfs(5, 5), Distance::Reachable(0));
    assert_eq!(graph.bfs(5, 0), Distance::Unreachable);
    assert_eq!(graph.bfs(0, 5), Distance::Unreachable);
}

#[test]
fn bfs_two_components() {
    let graph = Graph {
        edges: vec![
            // First connectivity component
            vec![1],
            vec![0, 3],
            vec![3],
            vec![1, 2],
            // Second connectivity component
            vec![5, 6],
            vec![6, 4],
            vec![4, 5],
        ],
    };

    assert_eq!(graph.bfs(0, 2), Distance::Reachable(3));
    assert_eq!(graph.bfs(2, 0), Distance::Reachable(3));
    assert_eq!(graph.bfs(4, 5), Distance::Reachable(1));
    assert_eq!(graph.bfs(4, 0), Distance::Unreachable);
    assert_eq!(graph.bfs(0, 4), Distance::Unreachable);
}

#[test]
fn dfs_simple() {
    let graph = Graph {
        edges: vec![
            vec![1, 2, 3],
            vec![2, 0],
            vec![0, 1],
            vec![0, 4],
            vec![1, 3],
            vec![],
        ],
    };

    assert_eq!(graph.dfs(0, 4), Some(vec![0, 3, 4]));
    assert_eq!(graph.dfs(1, 3), Some(vec![1, 2, 0, 3]));
    assert_eq!(graph.dfs(0, 5), None);
}

#[test]
fn dfs_two_components() {
    let graph = Graph {
        edges: vec![
            // First connectivity component
            vec![1],
            vec![0, 3],
            vec![3],
            vec![1, 2],
            // Second connectivity component
            vec![5, 6],
            vec![6, 4],
            vec![4, 5],
        ],
    };

    assert_eq!(graph.dfs(2, 0), Some(vec![2, 3, 1, 0]));
    assert_eq!(graph.dfs(2, 4), None);
}