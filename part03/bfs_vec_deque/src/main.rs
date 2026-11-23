use std::collections::VecDeque;

/// 그래프의 인접 리스트 표현
/// graph[u] -> u에서 갈 수 있는 노드 목록
fn bfs(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut order = Vec::new();
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);

        for &next in graph[node].iter() {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    order
}

fn main() {
    // 그래프
    // 0 - 1 - 3
    // |   |
    // 2 - 4
    let graph = vec![vec![1, 2], vec![0, 3, 4], vec![0, 4], vec![1], vec![1, 2]];

    let order = bfs(&graph, 0);
    println!("BFS 방문 순서: {:?}", order);
}
