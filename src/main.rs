fn main() {
    // let (angka1, angka2) = (15.0, 4.0);
    let (angka1, angka2): (f32, f32) = (15.0, 4.0);

    let tambah = angka1 + angka2;
    let kurang = angka1 - angka2;
    let bagi = angka1 / angka2;
    let kali = angka1 * angka2;
    let sisa = angka1 % angka2;

    println!("{} + {} = {}", angka1, angka2, tambah);
    println!("{} - {} = {}", angka1, angka2, kurang);
    println!("{} / {} = {}", angka1, angka2, bagi);
    println!("{} * {} = {}", angka1, angka2, kali);
    println!("{} % {} = {}", angka1, angka2, sisa);
}
