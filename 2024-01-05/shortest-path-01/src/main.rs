use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{stdin, Read}; // Rust는 최대힙으로 동작함.
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    // 노드의 갯수
    let n = input.next().unwrap().parse::<usize>().unwrap();
    // 간선의 갯수
    let m = input.next().unwrap().parse::<usize>().unwrap();

    // 시작 노드 번호 입력 받기
    let start = input.next().unwrap().parse::<usize>().unwrap();
    //각 노드에 연결되어 있는 노드에 대한 정보를 담는 리스트
    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n + 1];
    // 최단 거리 테이블을 모두 무한으로 초기화
    let mut distance = vec![i32::MAX; n + 1];

    println!("distance: {:?}", distance);

    // 모든 간선 정보 입력 받기
    for _ in 0..m {
        let a = input.next().unwrap().parse::<usize>().unwrap();
        let b = input.next().unwrap().parse::<usize>().unwrap();
        let c = input.next().unwrap().parse::<usize>().unwrap();

        graph[a].push((b, c))
    }

    println!("graph: {:?}", graph);

    dijkstra(start, &mut distance, graph);

    for i in 1..n + 1 {
        if distance[i] == i32::MAX {
            println!("INFINITY")
        } else {
            println!("{}", distance[i])
        }
    }
}

fn dijkstra(start: usize, distance: &mut Vec<i32>, graph: Vec<Vec<(usize, usize)>>) {
    let mut q = BinaryHeap::new();
    // 시작 노드로 가기 위한 최단 경로는 0으로 설정하여 큐에 넣음
    q.push(Reverse((0, start)));
    distance[start] = 0;

    println!("q: {:?}", q);

    while !q.is_empty() {
        // 가장 최단 거리가 짧은 노드에 대한 정보 꺼내기
        let Reverse((dist, now)) = q.pop().unwrap();

        // 현재 노드가 이미 처리된 적이 있는 노드라면 무시
        if distance[now] < dist {
            continue;
        }

        // 현재 노드와 연결된 다른 인접한 노드들을 확인
        for i in &graph[now] {
            let cost = dist + i.1 as i32;
            // 현재 노드를 거쳐서 다른 노드로 이동하는 거리가 더 짧은 경우
            if cost < distance[i.0] {
                distance[i.0] = cost;
                q.push(Reverse((cost, i.0)));
            }
        }
    }
}
