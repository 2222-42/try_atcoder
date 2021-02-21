use proconio::input;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
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
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as an `usize`, for a shorter implementation.
#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| 18446744073709551615).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: [(usize, usize, usize); m],
    }

    // println!("{}, {}", n, m);
    // println!("{:?}", q);
    // n行の出力をする

    // qのそれぞれの第一変数と第二引数は1からnのいずれか

    // i (>=1, <= n)について、iからiまでの経路を探す。
    // 存在しなかったら-1を、
    // 存在したら、そのうち、最小の合計値を出力する。

    // 方針
    // (1)iからi自身への道がないか探す。 -> 自明に最小の値になるので、それを採用する。

    // (2)ダイクストラ法を使う
    // i から iとは異なるjへの道を探し、
    // 次に、jからiへの道を探す。
    // (2-1)道を全て反転させ、iからjへの道を探す。

    let mut graph = vec![];
    let mut i = 0;
    while i < m {
        let v: Vec<Edge> = vec![];
        graph.push(v);
        i += 1;
    }

    for (node1, node2, cost) in q.into_iter() {
        // println!("{}, {}, {}", node1, node2, cost);
        graph[node1 - 1].push(Edge {
            node: node2 - 1,
            cost: cost,
        });
    }

    let mut answers = vec![0; n];

    let mut j = 0;
    while j < n {
        let mut k = 0;
        let mut sub_answers = vec![0; n];
        for self_checker in graph[j].iter() {
            if self_checker.node == j {
                answers[j] = self_checker.cost;
                j += 1;
                continue;
            }
        }

        while k < n {
            if k != j {
                let result = shortest_path(&graph, j, k);
                let sa = match result {
                    Some(n) => n,
                    None => 0,
                };

                if sa != 0 {
                    let result = shortest_path(&graph, k, j);
                    match result {
                        Some(n) => sub_answers[k] = sa + n,
                        None => sub_answers[k] = 0,
                    }
                }
            }
            k += 1;
        }

        // let filtered_sub_answers: Vec<usize> =
        //     sub_answers.iter().filter(|&&x| x != 0).cloned().collect();

        // let answer = filtered_sub_answers.into_iter().min();
        let answer_j = sub_answers.iter().filter(|&&x| x != 0).min();
        match answer_j {
            Some(n) => answers[j] = *n,
            None => answers[j] = 0,
        }

        j += 1;
    }

    for length in answers {
        if length > 0 {
            println!("{}", length);
        } else {
            println!("-1");
        }
    }
}
