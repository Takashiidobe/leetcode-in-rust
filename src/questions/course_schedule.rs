use std::collections::{HashMap, HashSet, VecDeque};

pub fn course_schedule(num_courses: usize, prerequisites: Vec<Vec<usize>>) -> bool {
    let mut prereqs = vec![HashSet::new(); num_courses];
    let mut courses = vec![HashSet::new(); num_courses];
    let mut courses_taken = HashSet::new();

    for prerequisite in prerequisites {
        let (course, prereq) = (prerequisite[0], prerequisite[1]);

        prereqs[prereq].insert(course);
        courses[course].insert(prereq);
    }

    let mut not_taken = Vec::new();

    for i in 0..num_courses {
        if courses[i].is_empty() {
            not_taken.push(i);
        }
    }

    let mut q = VecDeque::from(not_taken);

    while !q.is_empty() {
        let course = q.pop_front().unwrap();
        courses_taken.insert(course);

        for c in &prereqs[course] {
            courses[*c].remove(&course);
            if courses[*c].is_empty() {
                q.push_back(*c);
            }
        }
    }

    courses_taken.len() == num_courses
}
