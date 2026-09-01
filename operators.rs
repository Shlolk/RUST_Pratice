fn main(){
//arthematic operators 
let a = 23;
let b = 24 ;
let sum = a+b;
let sub = a-b;
let mul = a*b;
let div = a/b;
let rem = a%b;
println!("{},{},{},{},{}",sum,sub,mul,div,rem);

// comprasion operators 
  let is_proraming_fun:bool = true ;
  let is_fish_tasty:bool= false;
  println!("{}",is_programing_fun);
  println!("{}",is_fish_tasty);
  //boolan with comparison 
  let age = 20;
let can_vote = age >= 18;

println!("Can vote? {}", can_vote);

//Using Booleans in if Statements
let is_logged_in = true;

if is_logged_in {
  println!("Welcome back!");
} else {
  println!("Please log in.");
}
}
