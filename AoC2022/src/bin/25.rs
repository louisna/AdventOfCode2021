use num_bigint::BigUint;

fn main() {
    let content = include_str!("../../inputs/25.txt");
    let numbers: Vec<i64> = content.split('\n').map(convert_from_5_to_10).collect();

    println!("Decimal numbers: {:?}", numbers);

    let total_10: i64 = numbers.iter().sum();
    println!("Total in base 10: {}", total_10);
    
    let total_5 = convert_from_10_to_5(total_10);
    println!("Total is base 5 strange: {}", total_5);

    let some = 3;
    println!("{}", convert_from_10_to_5(some));

}

fn convert_from_5_to_10(line: &str) -> i64 {
    let nb = line.len() as i64;
    let mut res = 0;
    let chars: Vec<_> = line.chars().collect();
    for (idx, i) in (0..nb).rev().enumerate() {
        res += 5i64.pow(i as u32) * match chars[idx] {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unknown char"),
        };
    }

    res
}

fn convert_from_10_to_5(number: i64) -> String {

    let base_5 = BigUint::from(number as u64);
    let base_5 = base_5.to_str_radix(5);
    let n = base_5.len() + 1;

    let number = number + (5i64.pow(n as u32) - 1) / 2;

    // Construct the number.
    let a = BigUint::from(number as u64);
    let res = a.to_str_radix(5);

    res.replace("0", "=").replace("1", "-").replace("2", "0").replace("3", "1").replace("4", "2")
}