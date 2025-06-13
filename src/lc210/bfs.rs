pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;
    let mut in_degree = vec![0; num_courses];
    for pair in &prerequisites {
        in_degree[pair[0] as usize] += 1;
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
        for pair in &prerequisites {
            if i == pair[1] as usize {
                in_degree[pair[0] as usize] -= 1;
                if in_degree[pair[0] as usize] == 0 {
                    queue.push_back(pair[0] as usize);
                }
            }
        }
    }
    if res.len() == num_courses {
        res
    } else {
        vec![]
    }
}
