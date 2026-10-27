fn main() {
    let z: u32 = 10;
    let x: u32 = {
        let y: u32 = 1;
        dbg!(y);
        z - y
    };
    dbg!(x);
    dbg!(y);
}

