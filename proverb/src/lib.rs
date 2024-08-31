pub fn build_proverb(list: &[&str]) -> String {
    let count = list.len();
    let mut result = String::new();

    for i in 0..count {
        if i == count - 1 {
            result.push_str(&format!("And all for the want of a {}.", list[0]));
            break;
        }

        result.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            list[i],
            list[i + 1]
        ));
    }

    result
}
