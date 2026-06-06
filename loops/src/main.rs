

fn main(){
    println!("this is a cool rust project and i am about to see how in");

    let num = 34;

    if num < 20 {
        println!("This is unfortunate");
    }
    else {
        println!("this is interesting and we know");
    }

    let mut counter = 1;

    loop {
        if counter >= 10{
            break;
        }
        println!("This is funny right");
        println!("counter is {counter} ");
        counter = counter + 1;
    }

    let mut newcount = 0;
    'counting: loop{
        let mut remainder = 10;
        println!("counter = {newcount}");

        loop {
            println!("remaining is = {remainder}");
            if remainder == 9{
                break;
            }
            if newcount == 2{
                break 'counting;
            }

            remainder = remainder - 1;
        }
        newcount += 1;
    }

    println!("end of counting is = {newcount}");


    let mut iter = 1;

    'time_tabel: loop {
        println!("\nMultiplication Timetable for {iter}\n");
        let mut newIter = 1;

        loop{
            // println!("inter iter = {newIter}");
            if newIter > 10{
                break
            }
            let mul = iter*newIter;
            println!("{iter} X {newIter} = {mul}");
            newIter += 1;


        }


        iter += 1;
        if iter > 100{
            break
        }

    }


    let arr = [340,440,550,440,56,53,4];

    let mut index = 0;


    while index<5 {
        println!("the value at index {index} is {}",arr[index]);
        index = index + 1;
    }

    for el in arr{
        println!("Using the loop the value is, {el}");
    }

    for num in (1..6).rev() {
        println!("{num}")
    }



}