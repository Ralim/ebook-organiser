pub fn flip_comma_split(val: String) -> String {
    if val.contains(',') {
        let mut parts: Vec<&str> = val.split(',').collect();
        parts.reverse();
        parts.join(" ")
    } else {
        val
    }
}
