fn print_square (number:i32)-> i32
{  let mut col = 0;
    while col < number {
        let mut row = 0;
        while row < 10 {
            print!("{}", if row == col { "X" } else { "O" });
            col += 1;
        }
        println!("");
        col += 1;
    }
    return col
}





fn main() {
    print_square(10);
}