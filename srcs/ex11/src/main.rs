fn rot(n: u32, mut x: u32, mut y: u32, rx: u32, ry: u32) -> (u16, u16) {
    if ry == 0 {
        if rx == 1 {
            x = n - 1 - x;
            y = n - 1 - y;
        }
        return (y as u16, x as u16);
    }
    (x as u16, y as u16)
}

fn reverse_map(d: f64) -> (u16, u16) {
    // inverse hilbert curve
    let mut rx: u32;
    let mut ry: u32;
    let mut t: u32 = (d * 4294967295.) as u32;
    // number of iterations
    let n: u32 = 65536;
    let mut s: u32 = 1;
    let mut coord: (u16, u16) = (0, 0);

    while s < n {
        rx = 1 & (t / 2);
        ry = 1 & (t ^ rx);
        coord = rot(s, coord.0 as u32, coord.1 as u32, rx, ry);
        coord.0 += (s * rx) as u16;
        coord.1 += (s * ry) as u16;
        t /= 4;
        s *= 2;
    }
    coord
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_map() {
        let size: u32 = 999;
        let mut save = vec![];
        for d in 0..size {
            let tmp = reverse_map(d as f64 * 0.001);
            save.push(vec![tmp.0, tmp.1]);
        }

        println!("{:?}", save);
        let s = save.len();
        save.sort_by(|a, b| a.partial_cmp(b).unwrap());
        save.dedup();
        assert_eq!(s, save.len());
        assert_eq!(s as u32, size);
    }
    
    fn rot_test(n: u32, mut x: u32, mut y: u32, rx: u32, ry: u32) -> (u32, u32) {
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
            coord = rot_test(n, coord.0, coord.1, rx, ry);
            s /= 2;
        }
        d as f64 / 4294967295.
    }
    
    #[test]
    fn eval_map1() {
        let d = 0.;
        let tmp = reverse_map(d);
        assert_eq!(tmp, (0, 0));
        assert_eq!(tests::map(tmp.0, tmp.1), d)
    }
    
    #[test]
    fn eval_map2() {
        let d = 1.;
        let tmp = reverse_map(d);
        assert_eq!(tmp, (65535, 0));
        assert_eq!(tests::map(tmp.0, tmp.1), d)
    }
    
    #[test]
    fn eval_map3() {
        let d = 0.3333333333333333;
        let tmp = reverse_map(d);
        assert_eq!(tmp, (0, 65535));
        assert_eq!(tests::map(tmp.0, tmp.1), d)
    }
    
    #[test]
    fn eval_map4() {
        let d = 0.6666666666666666;
        let tmp = reverse_map(d);
        assert_eq!(tmp, (65535, 65535));
        assert_eq!(tests::map(tmp.0, tmp.1), d)
    }
    
    #[test]
    fn eval_map5() {
        let d = 7.878104226635328e-5;
        let tmp = reverse_map(d);
        assert_eq!(tmp, (120, 876));
        assert_eq!(tests::map(tmp.0, tmp.1), d)
    }
    
    #[test]
    fn eval_map6() {
        let d = 2.0318664615116703e-5;
        let tmp = reverse_map(d);
        assert_eq!(tmp, (489, 3));
        assert_eq!(tests::map(tmp.0, tmp.1), d)
    }
}

fn main() {
    let d: f64 = 0.58;

    println!("{d} = {:?}", reverse_map(d));
}
