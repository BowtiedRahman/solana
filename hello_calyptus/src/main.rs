use std::collections::HashMap;
fn main() {
    let unsigned: u32 = 677;
    let signed: i16 = -78;
    let float: f32 = 0.87;
    let a: i32 = 10;
    let b: i32 = 15;
    let character: char = 'c';
    println!("Character {}", character);
  // boolis is for true and false values  
    let boolean: bool = true;
    println!("boolean {}", boolean);
  // tuple is used to group different data types
    let tuple: (i32, f32, u16, bool) = (-55, 5.66, 66, false);
    println!("tuple {:?}", tuple);
    // array is used to group elements of the same data type,it's not dynamic 
    // {:?} is used to print all elements in an array 
    let array: [char; 8] = ['b', 'o', 'w', 't', 'i', 'e', 'd', 'R' ];
    println!("array of characters: {:?} ", array);
    //vector is used to group elements ofthe same data types but is dynamic 
    let mut vector: Vec<String> = vec!
    [String::from("hello"),
    String::from("bow"),
    String::from("tied"),
    String::from("jungle"),];
    println!("vector: {:#?} ", vector);
    //string is used for grouping characters
    let greet: &str = "hello world";
    println!("greeting {}", greet);
    //hash map are like dictionaries used to group different data types
    let mut names_class: HashMap<&str, i32> = std::collections::HashMap::new(); 
    names_class.insert("P1", 8);
    names_class.insert("P2", 7);
    println!("names of class and students {:#?}", names_class);
   // struct students {
     //   Name: String,
       // Age:  i32
  //  };
    // println!("Different numbers =>, {:?}, {}, {}", unsigned, signed, float);
    // println!("Hello, World!, {} {}",a,b);
    // println!("Hello, world!");

   // aninfinite loopthat keeps executing unless a break statement is encountered
   //suitable if the number of iteration is unknown or you want a block of code
   //to execute infinitely
   let mut num: i32 = 0;
    loop {
        println!("looping>>>");
        num += 1;
        if num >= 7 {
            break;
        }
    }

    //while the condition is true it will keep executing until the conditionis false
    let mut num2: i32 = 0;
    while num2 < 10 {
        println!("i = {}", num2);
        num2 += 1;
    }

 //   for ii in 0..10 {
   //     println!("hello world {}", ii);
    //}

    //looping over an array
    let numbers: [i32; 8] = [0,1,-2,5,-8,8,6,7,];
    for i in numbers.iter() {
        println!("element: {}", i);
    }

    //looping over an iterator and array
    let nums: [f32; 5] = [2.3,3.3,3.3,2.2,3.3];
    for (index, value) in nums.iter().enumerate() {
        println!("value at position {}: {}", index, value);
    }
    fn my_name(name: &str) -> String {
        format!("hello {}", name)
    }

   println!("{}", my_name("bowtiedrahman"))



}
