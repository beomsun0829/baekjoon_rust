use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn get_vec_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
{
    get_line()
        .split_whitespace()
        .filter_map(|x| x.parse::<T>().ok())
        .collect()
}

fn main() {
    println!("--- 프로그램 시작 ---");

    let n: usize = get_line().parse().unwrap();
    let m: usize = get_line().parse().unwrap();
    println!("도시 개수 (N): {}", n);
    println!("버스 노선 개수 (M): {}", m);

    // matrix: 인접 리스트. matrix[u]는 (v, weight) 튜플의 벡터.
    // u에서 v로 가는 가중치 weight인 간선이 있음을 의미.
    let mut matrix: Vec<Vec<(usize, usize)>> = vec![vec![]; n];

    println!("\n--- 버스 노선 입력 ---");
    for i in 0..m {
        let line = get_vec_line::<usize>();
        let u_from = line[0] - 1; // 0-indexed로 변환
        let v_to = line[1] - 1;   // 0-indexed로 변환
        let weight = line[2];
        matrix[u_from].push((v_to, weight));
        println!(
            "{}번째 노선: {} -> {} (비용: {})",
            i + 1,
            u_from,
            v_to,
            weight
        );
    }
    println!("인접 리스트 상태: {:?}", matrix);

    let line = get_vec_line::<usize>();
    let start = line[0] - 1; // 0-indexed로 변환
    let end = line[1] - 1;   // 0-indexed로 변환
    println!("\n시작 도시: {} (0-indexed)", start);
    println!("도착 도시: {} (0-indexed)", end);

    // dist: 시작 노드부터 각 노드까지의 현재까지 발견된 최단 거리.
    // usize::MAX는 '무한대'를 의미하며, 아직 도달하지 못했음을 나타냄.
    let mut dist: Vec<usize> = vec![usize::MAX; n];
    println!("\ndist 배열 초기화: {:?}", dist);

    // pq: 우선순위 큐 (BinaryHeap). Reverse 튜플을 사용하여 최소 힙처럼 동작.
    // (거리, 노드_번호) 형태로 저장되며, 거리가 가장 작은 요소가 우선적으로 pop 됨.
    let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new(); //(거리, 노드번호)

    // 시작 노드의 거리는 0으로 설정하고 큐에 삽입.
    dist[start] = 0;
    pq.push(Reverse((0, start)));
    println!("pq에 (0, {}) 삽입. pq 상태: {:?}", start, pq);
    println!("dist 배열 업데이트: {:?}", dist);

    println!("\n--- 다익스트라 알고리즘 시작 ---");
    let mut step_count = 0;

    // 큐가 비어있지 않은 동안 반복
    while let Some(Reverse((d_current, u_current))) = pq.pop() {
        step_count += 1;
        println!("\n--- Step {} ---", step_count);
        println!(
            "pq에서 꺼낸 요소: (거리: {}, 노드: {})",
            d_current, u_current
        );

        // 현재 큐에서 꺼낸 거리(d_current)가 이미 dist[u_current]에 기록된 거리보다 크다면
        // 이는 이미 u_current까지 더 짧은 경로를 찾아서 처리했으므로, 이 경로는 무시합니다.
        // (같은 노드가 큐에 여러 번 들어갈 수 있기 때문에 발생하는 중복 처리 방지)
        if d_current > dist[u_current] {
            println!(
                "  > {}까지의 현재 거리 ({})가 이미 확정된 거리 ({})보다 깁니다. 이 경로는 스킵합니다.",
                u_current, d_current, dist[u_current]
            );
            println!("  현재 dist 배열: {:?}", dist);
            continue;
        }

        // 현재 노드 u_current가 목표 노드(end)라면, 최단 경로를 찾았으므로 종료
        if u_current == end {
            println!(
                "  > 목표 노드 {}에 도달했습니다. 최단 거리: {}",
                end, d_current
            );
            println!("--- 다익스트라 알고리즘 종료 ---");
            println!("최종 최단 거리: {}", d_current);
            break;
        }

        println!(
            "  > 노드 {}의 이웃을 탐색합니다. (현재까지 확정된 {}까지의 거리: {})",
            u_current, u_current, d_current
        );

        // 현재 노드 u_current와 연결된 모든 이웃 노드들을 탐색
        // (v: 이웃 노드, weight: u_current에서 v까지의 간선 가중치)
        for &(v_neighbor, weight) in &matrix[u_current] {
            // u_current를 거쳐 v_neighbor로 가는 새로운 경로의 총 거리
            let new_dist = d_current + weight;
            println!(
                "    - 이웃 노드 {}: {} -> {} (기존 거리: {}), 간선 비용: {}",
                v_neighbor, u_current, v_neighbor, dist[v_neighbor], weight
            );

            // 만약 새로운 경로(new_dist)가 기존에 알려진 v_neighbor까지의 최단 거리(dist[v_neighbor])보다 짧다면 갱신
            if dist[v_neighbor] > new_dist {
                println!(
                    "      > {}까지의 거리가 {}에서 {}로 갱신됩니다.",
                    v_neighbor, dist[v_neighbor], new_dist
                );
                dist[v_neighbor] = new_dist; // dist 배열 갱신
                pq.push(Reverse((new_dist, v_neighbor))); // 갱신된 노드를 큐에 추가
                println!(
                    "      > pq에 (거리: {}, 노드: {}) 삽입. pq 상태: {:?}",
                    new_dist, v_neighbor, pq
                );
                println!("      현재 dist 배열: {:?}", dist);
            } else {
                println!(
                    "      > {}까지의 새 경로 ({})가 기존 경로 ({})보다 길거나 같으므로 갱신하지 않습니다.",
                    v_neighbor, new_dist, dist[v_neighbor]
                );
            }
        }
    }

    // 만약 break 문 없이 큐가 모두 비워졌다면 (목표 노드에 도달하지 못한 경우)
    if dist[end] == usize::MAX {
        println!("\n--- 다익스트라 알고리즘 종료 ---");
        println!("목표 노드 {}에 도달할 수 없습니다. 출력: 0", end);
        // 문제 요구사항에 따라 도달 불가능할 때 0을 출력해야 하므로
        println!("{}", 0);
    }
}