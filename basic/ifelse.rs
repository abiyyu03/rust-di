fn ngecek_variable(n: i64) -> String {
    if n > 20 as i64 {
        return "Gacor kang".to_string();
    }

    return "Woylah cik !".to_string();
}

fn ngecek_jeneng(name: String) -> String {
    if name == "Abiyyu" {
        return "Hi {name}";
    } else if name == "Wildan"{
        return "Hi juga {name}";
    }

    return "Gakenal guweh";
}

fn main() {
    println!("Hasil pengecekan angka: {}",ngecek_variable(40));
    println!("Hasil cek nama: {}",ngecek_jeneng("Abiyyu".to_string()));
}
