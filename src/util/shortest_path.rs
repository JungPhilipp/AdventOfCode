#![allow(unused)]
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<Node> {
    cost: i64,
    position: Node,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl<Node> Ord for State<Node>
where
    Node: PartialEq + Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl<Node> PartialOrd for State<Node>
where
    Node: PartialEq + Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Edge<Node> {
    pub node: Node,
    pub cost: i64,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
pub fn shortest_path<Node>(
    adj_list: &HashMap<Node, Vec<Edge<Node>>>,
    start: &Node,
    goal: &Node,
) -> Option<i64>
where
    Node: Ord + Hash + Clone + Debug,
{
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = HashMap::<Node, i64>::new();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start.clone(), 0);
    heap.push(State {
        cost: 0,
        position: start.clone(),
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == *goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if let Some(distance) = dist.get(&position) {
            if cost > *distance {
                continue;
            }
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        if let Some(neighbors) = adj_list.get(&position) {
            for edge in neighbors {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node.clone(),
                };

                if let Some(distance) = dist.get_mut(&next.position) {
                    // If so, add it to the frontier and continue
                    if next.cost < *distance {
                        // Relaxation, we have now found a better way
                        *distance = next.cost;
                        heap.push(next);
                    }
                } else {
                    dist.insert(next.position.clone(), next.cost);
                    heap.push(next);
                }
            }
        }
    }

    // Goal not reachable
    None
}

fn main() {
    // This is the directed graph we're going to use.
    // The node numbers correspond to the different states,
    // and the edge weights symbolize the cost of moving
    // from one node to another.
    // Note that the edges are one-way.
    //
    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           10      |               |
    //                   +---------------+
    //
    // The graph is represented as an adjacency list where each index,
    // corresponding to a node value, has a list of outgoing edges.
    // Chosen for its efficiency.
    let graph = vec![
        // Node 0
        vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
        // Node 1
        vec![Edge { node: 3, cost: 2 }],
        // Node 2
        vec![
            Edge { node: 1, cost: 1 },
            Edge { node: 3, cost: 3 },
            Edge { node: 4, cost: 1 },
        ],
        // Node 3
        vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
        // Node 4
        vec![],
    ]
    .into_iter()
    .enumerate()
    .collect::<HashMap<_, _>>();

    assert_eq!(shortest_path(&graph, &0, &1), Some(1));
    assert_eq!(shortest_path(&graph, &0, &3), Some(3));
    assert_eq!(shortest_path(&graph, &3, &0), Some(7));
    assert_eq!(shortest_path(&graph, &0, &4), Some(5));
    assert_eq!(shortest_path(&graph, &4, &0), None);
}
