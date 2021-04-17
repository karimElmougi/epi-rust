pub fn normalize(path_name: &str) -> String {
    let mut path = vec![];

    if path_name.starts_with('/') {
        path.push("");
    }

    for path_segment in path_name.split('/') {
        match path_segment {
            "." | "" => continue,
            ".." => match path.last() {
                None | Some(&"..") => path.push(".."),
                Some(&"") => {}
                _ => {
                    path.pop();
                }
            },
            _ => path.push(path_segment),
        }
    }

    path.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        assert_eq!(normalize("sc//./../tc/awk/././"), "tc/awk");
        assert_eq!(normalize("/./awk"), "/awk");
        assert_eq!(normalize("/./../awk"), "/awk");
    }
}
