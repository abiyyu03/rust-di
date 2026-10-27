fn penjumlahan(a: i32, b: i64, c: i128) -> i32{
    let b_casted: i32 = b as i32;
    let c_casted: i32 = c as i32;
    return a + b_casted + c_casted;
}

fn pengurangan(a: i64, b: i64) -> i64 {
    a - b
}

fn fibbonaci(n: u32) -> u32 {
    if n < 2 {
        return n; 
    } else {
        return fibbonaci(n - 1) + fibbonaci(n - 2);
    }
}

fn main() {
    let name: &str = "Wildan";
    println!("Belajar penjumlahan: {}", penjumlahan(1,2,3));
    println!("Belajar pengurangan: {}", pengurangan(2,4));
    println!("Name dia :{name}");
    println!("Tes Fibonaci: {}",fibbonaci(3));
}
