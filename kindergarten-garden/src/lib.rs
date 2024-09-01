fn index_of(student: &str) -> usize {
    student.chars().next().unwrap() as usize - b'A' as usize
}

fn get_plant(c: char) -> &'static str {
    match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => panic!("unknown plant: {}", c),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = index_of(student) * 2;

    diagram
        .lines()
        .flat_map(|line| line[index..=index + 1].chars().map(|c| get_plant(c)))
        .collect()
}
