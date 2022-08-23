let a = 1;
let b = 2;
let c = 3;

let a = 99;
if a > 99 {
    println!("Big number");
} else {
    println!("Small number");
}

let b = 99;
if b > 99 {
    if b > 200{
        println!("Huge number");
    } else {
        println!("Big number");
    }
} else {
    println!("Small number");
}

let c = 99;
if c > 200{
    println!("Huge number");
}else if c > 99 {
    println!("Big number");
} else {
    println!("Small Number");
}
