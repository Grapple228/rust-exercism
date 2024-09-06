use std::collections::{BTreeMap, BinaryHeap, HashSet};

#[allow(clippy::new_without_default)]
pub struct School {
    grades: BTreeMap<u32, BinaryHeap<String>>,
    students: HashSet<String>,
}

impl School {
    pub fn new() -> School {
        Self {
            grades: BTreeMap::new(),
            students: HashSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.students.contains(student) {
            return;
        }

        self.students.insert(student.to_string());

        self.grades
            .entry(grade)
            .or_insert(BinaryHeap::new())
            .push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(&grade)
            .cloned()
            .unwrap_or(BinaryHeap::new())
            .into_sorted_vec()
    }
}
