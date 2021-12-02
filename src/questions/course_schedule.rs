pub fn course_schedule(num_courses: usize, prerequisites: Vec<Vec<usize>>) -> bool {
    let mut v: Vec<(usize, Vec<usize>)> = vec![(0, vec![]); num_courses];
    for p in prerequisites.iter() {
        v[p[0]].1.push(p[1]);
        v[p[1]].0 += 1;
    }
    let mut stack: Vec<usize> = Vec::new();
    for (i, e) in (0..).zip(v.iter()) {
        if e.0 == 0 {
            stack.push(i);
        }
    }
    let mut count = 0;
    while let Some(last) = stack.pop() {
        count += 1;
        for i in v[last].1.clone() {
            v[i].0 -= 1;
            if v[i].0 == 0 {
                stack.push(i);
            }
        }
    }
    count == num_courses
}
