fn main() {
    let content = include_str!("../../inputs/08.txt");
    let lines: Vec<&str> = content.split("\n").collect();
    let m = lines.len();
    let n = lines[0].len();
    let mut visible = vec![vec![false; n]; m];
    let matrice: Vec<Vec<i8>> = lines.iter().map(|line| line.chars().map(|c| c as i8).collect()).collect();

    let mut total = 0;

    // Left -> right.
    for i in 0..m {
        let mut previous = -1;
        for j in 0..n {
            let value = matrice[i][j];
            if value > previous && !visible[i][j] {
                visible[i][j] = true;
                total += 1;
            }
            previous = value.max(previous);
        }
    }

    // Right -> left.
    for i in 0..m {
        let mut previous = -1;
        for j in (0..n).rev() {
            let value = matrice[i][j];
            if value > previous && !visible[i][j] {
                visible[i][j] = true;
                total += 1;
            }
            previous = value.max(previous);
        }
    }

    // Top -> down.
    for j in 0..n {
        let mut previous = -1;
        for i in 0..m {
            let value = matrice[i][j];
            if value > previous && !visible[i][j] {
                visible[i][j] = true;
                total += 1;
            }
            previous = value.max(previous);
        }
    }

    // Down -> top.
    for j in 0..n {
        let mut previous = -1;
        for i in (0..m).rev() {
            let value = matrice[i][j];
            if value > previous && !visible[i][j] {
                visible[i][j] = true;
                total += 1;
            }
            previous = value.max(previous);
        }
    }

    println!("Res 1: {}", total);

    // Part 2.
    let mut best: u64 = 0;
    for i in 0..m {
        for j in 0..n {
            let value = matrice[i][j];
            
            // Up.
            let mut c_up = 0;
            for i2 in (0..i).rev() {
                c_up += 1;
                if matrice[i2][j] >= value {
                    break;
                }
            }

            // Down.
            let mut c_down = 0;
            for i2 in i + 1..m {
                c_down += 1;
                if matrice[i2][j] >= value {
                    break;
                }
            }

            // Left.
            let mut c_left = 0;
            for j2 in (0..j).rev() {
                c_left += 1;
                if matrice[i][j2] >= value {
                    break;
                }
            }

            // Right.
            let mut c_right = 0;
            for j2 in j + 1..n {
                c_right += 1;
                if matrice[i][j2] >= value {
                    break;
                }
            }

            best = best.max(c_up * c_down * c_left * c_right);
        }
    }

    println!("Res 2: {}", best);
}