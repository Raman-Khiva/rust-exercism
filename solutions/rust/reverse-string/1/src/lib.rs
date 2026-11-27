pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");

    let mut ans = String::new();

    for c in input.chars().rev() {
        ans.push(c);
    }
    ans
}
