fn roman_as_num(roman: &str) -> u64 {
    use std::collections::HashMap;

    let roman_values: HashMap<char, u64> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut total = 0; 
    let mut prev_value = 0; 
    
    for char in roman.chars().rev() {
        let current_value = *roman_values.get(&char).unwrap_or(&0); 

        if current_value < prev_value {
            total -= current_value;
        } else {
            total += current_value;
        }
        prev_value = current_value; 
    }
    total
}
