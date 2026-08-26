fn main () {
    let x : i32 =5;
    let y : i32 ;
    assert_eq!(x,5);
    println!("success");
}
//in the first its show the out but due the exrat assign the varible like y where we are not assign it showing a sweet waring on that so 
// we gonna fix that by using the underscore it meams to use because it help to understand compiler that igonre that thing
//here can use in the below code how to use it 

fn main (){
    let x :i31 = 10;
    let _y:i32;// here complier ignore this delecared variable .....
    assert_eq!(x,10);
    println!("succes");
// here we are gonna learn about mute means mutiable 
// in rust varibles are immutaible so we mut to that later on we can the change the value of varibale 

fn main (){
    let mut x :i32 = 90; // here we using mut to making varibale mutaible 
    x+=2; 
    println!("{}",x)
// lets talk abot assert_eq! function it compare the variables with same data type
        fn main (){
        let mut  x:i32 = 100;
        x+=2;
        assert_eq!(x,102); // using this to compare the value ....
        println!("yes its right");
    
