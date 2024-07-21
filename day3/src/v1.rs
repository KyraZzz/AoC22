fn v1() {
    let res = include_str!("input.txt")
        .lines()
        .map(|line| {
            let length = line.len();
            let char_list = line.chars().collect::<Vec<_>>();
            let (first_half, second_half) = (
                char_list[0..length / 2].to_vec(),
                char_list[length / 2..].to_vec(),
            );
            let common_char = first_half
                .iter()
                .find(|c| second_half.contains(c))
                .expect("Need to find one and only one common character in two strings")
                .to_owned();
            common_char
        })
        .collect::<Vec<_>>();
    let sum = res.iter().fold(0, |acc, &x| {
        let x_utf = x as u32;
        let lower_a_utf = 'a' as u32;
        let upper_a_utf = 'A' as u32;
        let priority = if x.is_uppercase() {
            27 + x_utf - upper_a_utf
        } else {
            1 + x_utf - lower_a_utf
        };
        acc + priority
    });
    println!("{:?}", sum);
}
