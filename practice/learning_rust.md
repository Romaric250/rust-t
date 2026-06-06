
panic!("hey lets end here") -> this is similar exit in python. Anything that comes after that will never run



0.1 Variables and Immutability

By default variables in rust are immutable, This means they cannot be modified once created. 

This take care of safety and easy concurrency

You declear variables using the let keyword, and add the prefix mut if you want to remove the default immutability
that rust provides. 

To declear a constant, you use the const keyword, and its important to note that const's by default are meant to
be always immutable. if you think about it for sometime, you will understand that that is actually what a const is supposed to do. Rust is also very amazing for the immutability aspect of variables. 

its a way for to use the power of concurrency that rust provides us. 

Its important to note that const variables have to be annotated. const can also be decleard in any scope making them very useful. 

const may be set only to a constant expression and not to an expression that may change at run time.



0.2 Shadowing

    You can declear a variable with the name exactly the same as the previous variable. Rustaceans say that the first
    variable is shadowed by the second, which means is the second variable that the compiler will see when the variable is used. 

    in summary, the second variable overshadows the first untill it itself is overshadowed or the scope ends. 


0.3 I  can just use curly braces to separate scope without it being an if, or func declearatoin. 
its actually quit interesting. 



2. Data Types

    Rust has very common data types similar to other programming languages. 



    a. Integer 
        intergers can be either signed or unsigned. 
        a.1 Signed -> i8, i16,i32,i65,i128 
        a.2 Unsigned -> u8, u16, u32, u64m u128
        when declearing variables, since rust is statically typed, you have to declear the type upone creating the variable so that rust
        can know the operations that can be performed on that particular variable. 

    b. Floating point representation 
        Float also have two primitive floating point numbers 
        - f32 and f64
        - All floating point numbers by default are signed. 
        


theses are floats

variables, we have f64, f32, f64 holds 8 bytes of data, while f32 holds 4 bytes of data. so if you want more precission you should use f64.


Intergers in rust

- Integer sizes
i8 - 8bits
i16 - 16bits
i32 - 32 bts


unsigned integers the smallers value unsigned integer can only be 0.

u8
u16
u32
u64
u128
char is the same as u32 but is been validated to be a unicode.


converting numbers with the as, e.g y as i64


Booleans

we have two of them, true and false

booleans is a u8, 

so we can do true as u8 which will return 1
false as u8 -> 0


1 == 2 false



if cats > 1{
    println!("this is cool");
} else if cats > 1 {

}


statements and expressions
 
expressions evalautes to a value
e.g cats > 1_000 evaluates to a value. 



A statement does not evaluate to a value.

Does not evalaute to a vaule, it always ends with a semi column. 




Diving deeper into;

tuples, 
structs
arrrays



Tuples

units are empty tuples, they are used in function calls. 

