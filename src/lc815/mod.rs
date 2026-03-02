// 815. Bus Routes

use std::collections::{HashMap, HashSet, VecDeque};

/// 解决 LeetCode 815. 公交路线问题
/// - routes: 二维数组，每个子数组代表一条公交路线的站点列表
/// - source: 起点站点
/// - target: 终点站点
/// - 返回：最少乘坐的路线数，无法到达返回 -1
fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    // 边界条件：起点=终点，无需乘车
    if source == target {
        return 0;
    }

    // 1. 构建「站点→所属路线」的映射表
    let mut station_to_routes: HashMap<i32, Vec<usize>> = HashMap::new();
    for (route_idx, route) in routes.iter().enumerate() {
        for &station in route {
            station_to_routes
                .entry(station)
                .or_default()
                .push(route_idx);
        }
    }

    // 2. BFS 初始化：队列存储 (当前路线索引, 已坐路线数)
    let mut queue = VecDeque::new();
    // 记录已访问的路线（避免重复遍历）
    let mut visited_routes = HashSet::new();

    // 把所有包含起点的路线加入队列（多源BFS起点）
    if let Some(start_routes) = station_to_routes.get(&source) {
        for &route_idx in start_routes {
            queue.push_back((route_idx, 1));
            visited_routes.insert(route_idx);
        }
    }

    // 3. 开始 BFS 遍历路线
    while let Some((cur_route_idx, step)) = queue.pop_front() {
        // 遍历当前路线的所有站点
        for &station in &routes[cur_route_idx] {
            // 找到终点，直接返回当前步数
            if station == target {
                return step;
            }

            // 找到当前站点能换乘的所有路线
            if let Some(transfer_routes) = station_to_routes.get(&station) {
                for &transfer_route in transfer_routes {
                    // 未访问过的路线才加入队列
                    if !visited_routes.contains(&transfer_route) {
                        visited_routes.insert(transfer_route);
                        queue.push_back((transfer_route, step + 1));
                    }
                }
            }
        }
    }

    // 遍历完所有路线仍未找到终点
    -1
}

#[cfg(test)]
mod tests;
