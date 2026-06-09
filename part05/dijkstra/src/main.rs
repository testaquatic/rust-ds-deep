use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct State {
    cost: u64,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// 인접리스트 그래프
fn dijkstra(graph: &Vec<Vec<(usize, u64)>>, start: usize) -> Vec<u64> {
    let n = graph.len();
    let mut dist = vec![u64::MAX; n];
    dist[start] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        if cost > dist[node] {
            continue;
        }

        for &(next, weight) in &graph[node] {
            let next_cost = cost + weight;
            if next_cost < dist[next] {
                dist[next] = next_cost;
                heap.push(State {
                    cost: next_cost,
                    node: next,
                });
            }
        }
    }

    dist
}

fn main() {
    let mut graph = vec![vec![]; 5];
    graph[0].push((1, 4));
    graph[0].push((2, 1));
    graph[2].push((1, 2));
    graph[1].push((3, 1));
    graph[2].push((3, 5));
    graph[3].push((4, 3));

    let dist = dijkstra(&graph, 0);
    println!("0번 노드에서의 최단 거리");
    for (node, cost) in dist.iter().enumerate() {
        println!("    -> {} : {}", node, cost);
    }
}
