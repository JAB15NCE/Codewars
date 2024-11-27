fn tower_builder(n_floors: usize) -> Vec<String> {
    (0..n_floors)
        .map(|i|{
            let stars = "*".repeat(2 * i +1);
            let space = " ".repeat(n_floors - i - 1);
            format!("{}{}{}", space, stars, space)  
            })
    .collect()
}
