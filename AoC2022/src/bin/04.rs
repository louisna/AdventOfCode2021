fn main() {
    let content = include_str!("../../inputs/test.txt");
    let mut pairs: Vec<Vec<(i32, i32)>> = content
        .split("\n")
        .map(|line| {
            let tab = line.split(",");
            tab.map(|span| {
                let mut ext = span.split("-");
                (
                    ext.next().unwrap().parse::<i32>().unwrap(),
                    ext.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect()
        })
        .collect();

    let res = pairs.iter().fold(
        0,
        |score, pair| score + if (pair[0].0 <= pair[1].0 && pair[0].1 >= pair[1].1) || (pair[0].0 >= pair[1].0 && pair[0].1 <= pair[1].1) { 1 } else { 0 },
    );

    println!("Res 1: {}", res);

    pairs.iter_mut().for_each(|pair| pair.sort());
    let res = pairs.iter().fold(0, |score, pair| score + 
        if pair[0].1 >= pair[1].0 { 1 } else { 0 }
    );
    println!("Res 2: {}", res);
}
