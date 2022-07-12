fn triangle(n: i32) {
    let mut row = 1;
    while row <= n {
        let mut col = 1;
        while col <= row {
            print!("X");
            col += 1;
        }
        println!("");
        row += 1;
    }
}
 fn main() {
    triangle (10);
 }