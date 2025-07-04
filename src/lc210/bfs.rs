pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;
    let mut in_degree = vec![0; num_courses];
    let mut outs = vec![vec![]; num_courses];
    for pair in &prerequisites {
        in_degree[pair[0] as usize] += 1;
        outs[pair[1] as usize].push(pair[0] as usize);
    }
    let mut queue = std::collections::VecDeque::with_capacity(num_courses);
    let mut res = Vec::with_capacity(num_courses);
    for (i, &d) in in_degree.iter().enumerate() {
        if d == 0 {
            queue.push_back(i);
        }
    }
    while let Some(i) = queue.pop_front() {
        res.push(i as i32);
        for &j in &outs[i] {
            in_degree[j] -= 1;
            if in_degree[j] == 0 {
                queue.push_back(j);
            }
        }
    }
    if res.len() == num_courses {
        res
    } else {
        vec![]
    }
}
