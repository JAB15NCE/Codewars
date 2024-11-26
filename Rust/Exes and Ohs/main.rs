fn xo(s: &str) -> bool {
    let (mut x_count, mut o_count) = (0, 0);
    for c in s.to_lowercase().chars(){
        match c {
            'x' => x_count += 1,
            'o' => o_count += 1,
            _ => (),
        }
    }  
    x_count == o_count
}
