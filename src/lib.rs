pub fn rot13(s: &str) -> String {
    s.bytes()
        .map(|c| match c {
            b'a'..=b'z' => b'a' + ((c - b'a') + 13) % 26,
            b'A'..=b'Z' => b'A' + ((c - b'A') + 13) % 26,
            _ => c,
        } as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Wow I sure do love clippy";
        let correct = "Jbj V fher qb ybir pyvccl";
        assert_eq!(rot13(s), correct);
    }
}
