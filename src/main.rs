const LIMIT: usize = 100;

fn main() {
    let xs: i32 = 1;
    let mut counter: usize = 0;
    let mut x: [i32; LIMIT] = [0; LIMIT];

    loop {
        if counter == 0 {
            x[counter] = lcm(xs);
        } else {
            x[counter] = lcm(x[counter - 1]);
        }

        println!("{}. got random: {}", counter + 1, x[counter]);

        counter += 1;
        if counter == LIMIT {
            break;
        }
    }

    println!("Pembuktian kesamaan: ");
    for n in x.iter() {
        equal_check(n.clone(), x);
    }
}

// fungsi linear congruent method
// xs = x sebelumnya
// m = angka maksimal
// a, c = konstanta a untuk pengali dan c untuk range
// % = modulus operator dalam rust
fn lcm(xs: i32) -> i32 {
    let a: i32 = 3;
    let c: i32 = 15;
    let m: i32 = 2571;
    let x: i32 = (a * xs + c) % m;

    return x;
}

// pengecekan kesamaan
// counts = untuk menghitung kesamaan
// x = angka pembanding
// data = kumpulan angka acak hasil lcm
fn equal_check(x: i32, data: [i32; LIMIT]) {
    let mut counts: i32 = 0;
    for n in data.iter() {
        if x == n.clone() {
            counts += 1;
        }
    }

    // mengurangi dirinya sendiri
    counts -= 1;

    println!("Untuk angka {} ditemukan {} kesamaan", x, counts);
    return;
}