use std::cmp::max;

struct Zone {
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
}

impl Zone {
    /// 0 means it is in the zone
    /// -1: before the zone
    /// 1: past the zone
    fn is_in_zone(&self, x: i32, y: i32) -> i8 {
        if x >= self.minx && x <= self.maxx && y >= self.miny && y <= self.maxy {
            0
        } else if x > self.maxx || y < self.miny {
            1
        } else {
            -1
        }
    }
}

fn main() {
    let zone = Zone {
        minx: 241,
        maxx: 275,
        miny: -75,
        maxy: -49,
    };
    let mut total = 0;
    for vy in (-300..300).rev() {
        for vx in -300..300 {
            let tup = simulate(vx, vy, &zone);
            if tup.0 {
                if total == 0 {
                    println!("FOUND: {} {} with max {}", vx, vy, tup.1);
                }
                total += 1;
            }
        }
    }
    if total == 0 {
        println!("Not found in this range");
    } else {
        println!("Total found: {}", total);
    }
}

fn simulate(mut vx: i32, mut vy: i32, zone: &Zone) -> (bool, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut max_y: i32 = 0;
    let mut in_zone = zone.is_in_zone(x, y);
    while in_zone == -1 {
        max_y = max(max_y, y);
        x += vx;
        y += vy;
        vx -= if vx == 0 { 0 } else { 1 };
        vy -= 1;
        in_zone = zone.is_in_zone(x, y);
    }
    if in_zone == 0 {
        return (true, max_y);
    }
    (false, 0)
}
