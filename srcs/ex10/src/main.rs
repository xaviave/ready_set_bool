fn rot(n: u32, mut x: u32, mut y: u32, rx: u32, ry: u32) -> (u32, u32) {
    if ry == 0 {
        if rx == 1 {
            x = n - 1 - x;
            y = n - 1 - y;
        }
        return (y, x);
    }
    (x, y)
}

fn map(x: u16, y: u16) -> f64 {
    // hilbert curve
    let mut rx: u32;
    let mut ry: u32;
    let mut d: u32 = 0;
    // number of iterations
    let n: u32 = 65536;
    let mut s: u32 = n / 2;
    let mut coord = (x as u32, y as u32);
    while s > 0 {
        rx = ((coord.0 & s) > 0) as u32;
        ry = ((coord.1 & s) > 0) as u32;
        d += s * s * ((3 * rx) ^ ry);
        coord = rot(n, coord.0, coord.1, rx, ry);
        s /= 2;
    }
    d as f64 / 4294967295.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_map() {
        let size: u64 = 256;
        let mut save = vec![];
        for x in 0..size {
            for y in 0..size {
                save.push(map(x as u16, y as u16));
            }
        }

        let s = save.len();
        save.sort_by(|a, b| a.partial_cmp(b).unwrap());
        save.dedup();
        assert_eq!(s, save.len());
        assert_eq!(s as u64, size * size);
    }
}

fn main() {
    let x: u16 = 0;
    let y: u16 = 0;

    println!("{x} {y} = {}", map(x, y));
}
