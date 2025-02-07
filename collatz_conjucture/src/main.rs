use std::io ;

fn main() {
    println!("Welcome , here the 3x+1 formula can be tested ");
    println!("Enter the number ");
    let mut a = String::new();

    io::stdin()
   .read_line(&mut a)
   .expect("Failed to read line");

    let mut my_int:i32 = a.trim().parse().unwrap();

    println!("This is it's pattern");

   loop{
    if  my_int == 1 {
     break;
    }

  else if  my_int % 2 == 0 {
    my_int =  my_int / 2 ; 
    println!(" the number is {}" ,  my_int);
   
     }

   else if  my_int % 2 == 1 {
    my_int =  my_int * 3 + 1 ;
    println!(" the number is {}" ,  my_int);
    
     }
  }
  println!("Very intresting isn't it , it is actually unsolavble")
}