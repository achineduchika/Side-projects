use std::io;
fn addition(){
    let mut a = String::new();
   let mut b = String::new();
    println!("Enter the first number:");
   io::stdin().read_line(&mut a).unwrap();
   let a:i32 = a.trim().parse().unwrap();
   println!("Enter the second number:");
   io::stdin().read_line(&mut b).unwrap();
   let b:i32 = b.trim().parse().unwrap();
    let c = a + b;
    println!("\n{}+{}={}", a, b, c);
}
fn subtraction(){
    let mut a = String::new();
   let mut b = String::new();
    println!("Enter the first number:");
   io::stdin().read_line(&mut a).unwrap();
   let a:i32 = a.trim().parse().unwrap();
   println!("Enter the second number:");
   io::stdin().read_line(&mut b).unwrap();
   let b:i32 = b.trim().parse().unwrap();
    let c = a - b;
    println!("\n{}-{}={}", a, b, c);
}
fn multiplication(){
    let mut a = String::new();
   let mut b = String::new();
    println!("Enter the first number:");
   io::stdin().read_line(&mut a).unwrap();
   let a:i32 = a.trim().parse().unwrap();
   println!("Enter the second number:");
   io::stdin().read_line(&mut b).unwrap();
   let b:i32 = b.trim().parse().unwrap();
    let c = a * b;
    println!("\n{}×{}={}", a, b, c);
}
fn division(){
    let mut a = String::new();
   let mut b = String::new();
    println!("Enter the first number:");
   io::stdin().read_line(&mut a).unwrap();
   let a:i32 = a.trim().parse().unwrap();
   println!("Enter the second number:");
   io::stdin().read_line(&mut b).unwrap();
   let b:i32 = b.trim().parse().unwrap();
    let c = a/b;
    println!("\n{}÷{}={}", a, b, c);
}
fn main() {
   println!("Simple Calculator");
   let mut choice = String::new();
   
   println!("Pick the operator you want to perform +,-,× or ÷");
   io::stdin().read_line(&mut choice).unwrap();
   let choice:char = choice.trim().parse().unwrap();

   if choice == '+' {
   addition();
   }
   else if choice == '-' {
    subtraction();
   }
   else if choice == '×' {
    multiplication();
   }
   else if choice == '÷' {
    division();
   }
   else {
    println!("Please input a suitable operator");
   }
}
