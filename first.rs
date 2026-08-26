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
//here we gonna talk about scope...
//scope is the range within the program for which the item is valid 
        fn main (){
        let x :i32 = 25; // this varible is declared in the ourterscope  
            {
                let y : i32=500;// this delcared in the inner scope
                println!("the value of x {} and the value of y {}",x,y);
            }
            println!("value of the x {} and the value of y {}",x,y);
        }
// th3e above programe that i have mentioned is not gonna complier becuse a varible vaild inside scope only where it declared 
    //which is called scope delecaration...
