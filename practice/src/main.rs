// fn main(){
//     let num = 34;

//     let num = num + 30;
//     const THREE_HOURS_TO_SECONDS:u32 = 60*3*60;
//     println!("this is an immutable variable: {num}");

//     {
//         let num = 259;
//         println!("{num}");
//         println!("Testing scopes");
//     }

//     // num = 20;

//     println!("After Immutabilbity now is time, {num}");
//     println!("this is a constant here right now: {THREE_HOURS_TO_SECONDS}s")

// }

fn main(){
    let _num = 43;
    println!("This is before the panic");
    panic!("This is a panic");
    println!("This is a cool program");
}