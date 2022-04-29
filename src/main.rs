fn print_square (number:i32)-> i32
{  let mut row = 0;
    while row < number {
        let mut col = 0;
        while col < 10 {
            print!("{}", if row == col { "X" } else { "O" });
            col += 1;
        }
        println!("");
        row += 1;
    }
    return row
}





fn main() {
    print_square(10);
    print_square(15);
}