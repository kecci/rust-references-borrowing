// fn name_length(string: &String) -> usize {
//     // ----------------^
//     string.len()
// }
fn a_function(box_variable: Box<u32>) {
    println!("Box Variable {}", box_variable);
}
fn main() {
    // let my_name = String::from("Ali");
    //this ampersand is a reference
    // let len_name = name_length(&my_name);
    //  -----------------------^
    /*
        --> src/main.rs:5:32
        |
        5 |     let len_name = name_length(&my_name);
        |                                ^^^^^^^^
        |                                |
        |                                expected struct `std::string::String`, found `&std::string::String`
        |                                help: consider removing the borrow: `my_name`
    */

    // 
    let boxed_integer = Box::new(5);
    let reference_integer = &boxed_integer;

    println!("Boxed: {} and Reference {}", boxed_integer, reference_integer);
    /*
        As you see, when we send or assign a parameter, our assigned or passed values 
        starts with an ampersand. What would happens if we didn't use this ampersand? 
        It won't work. Because when we assigned boxed_integer to reference_integer, 
        compiler removed the boxed_integer variable.
    */

    // let box_variable = Box::new(6);
    // let reference_box_var = box_variable;
    // a_function(box_variable);
    /*
        error[E0382]: use of moved value: `box_variable`
        --> src/main.rs:9:16
        |
        6 |     let box_variable = Box::new(6);
        |         ------------ move occurs because `box_variable` has type `std::boxed::Box<u32>`, which does not implement the `Copy` trait
        7 |     let reference_box_var = box_variable;
        |                             ------------ value moved here
        8 |     
        9 |     a_function(box_variable);
        |                ^^^^^^^^^^^^ value used here after move

        error: aborting due to previous error; 1 warning emitted
    */

    // To solve this problem, we could create a new scope;
    let box_variable = Box::new(6);
    // a scope
    {
        let reference_box_var = &box_variable;
        println!("Solve new scope reference_box_var {}", reference_box_var);
    }
    a_function(box_variable);


    /* Mutable Borrow */


    // we can't create a mutable reference while the variable is immutable
    let mut a_number = 3;
    {
        let a_scoped_variable = &mut a_number;
        *a_scoped_variable +=1;
    }
    println!("A number is {}", a_number); // A number is 4

    let a_scoped_variable = &mut a_number;
    *a_scoped_variable +=1;


    // we can't create multiple mutable references
    let mut number = Box::new(5);

    let number2 = &mut number;
    println!("{}", number2);
    
    // let number3 = &mut number;
    // println!("{}, {}", number2, number3);
    /*
        Compiling playground v0.0.1 (/playground)
        error[E0499]: cannot borrow `number` as mutable more than once at a time
        --> src/main.rs:5:19
        |
      4 |     let number2 = &mut number;
        |                   ----------- first mutable borrow occurs here
      5 |     let number3 = &mut number;
        |                   ^^^^^^^^^^^ second mutable borrow occurs here
      6 |     
      7 |     println!("{}, {}", number2, number3);
        |                        ------- first borrow later used here
    */

    /*
        The benefit of having this restriction is that Rust can prevent data races at compile time. 
        A data race is similar to a race condition and happens when these three behaviors occur:

        1-) Two or more pointers access the same data at the same time.
        2-) At least one of the pointers is being used to write to the data.
        3-) There’s no mechanism being used to synchronize access to the data.
    */

    // We can fix this error by creating a new scope;
    let mut number = Box::new(5);
    let number2 = &mut number;
    println!("Number2 {}", number2);
    {
        let number3 = &mut number;
        println!("Number3 {}", number3);
    }


    // There is a similar rule like this rule when you're combining mutable and immutable references.
    // let mut number = Box::new(5);
    // let number2 = &number;
    // let number3 = &mut number;
    // println!("Number2 {} and Number3 {}", number2, number3);
    /*
        Compiling playground v0.0.1 (/playground)
        error[E0502]: cannot borrow `number` as mutable because it is also borrowed as immutable
        --> src/main.rs:5:19
        |
        4 |     let number2 = &number;
        |                   ------- immutable borrow occurs here
        5 |     let number3 = &mut number;
        |                   ^^^^^^^^^^^ mutable borrow occurs here
        6 |     
        7 |     println!("Number2 {} and Number3 {}", number2, number3);
        |                                           ------- immutable borrow later used here
    */

    // We can fix this code by this way;
    let mut number = Box::new(5);

    let number2 = &number;

    println!("Number2 {}", number2);

    let number3 = &mut number;

    println!("Number3 {}", number3);

    // Let's try to create a dangling reference and we'll sure Rust will prevent to compile your code.
    // let dangling_reference = a_func();

    // The solution is to return your data directly. Don't use ampersand.
    let dangling_reference = a_func();
    println!("dangling_reference {}", dangling_reference)
}

/* Dangling References */
// int *afunc();
// void main()
// {
//   int *pointer;
//   pointer = afunc();

//   fflush(stdin);
//   printf("%d", *pointer);
// }

// int * afunc()
// {
//   int x = 1000;
//   ++x;

//   return &x;
// }
/*
    warning: function returns the address of the local variable
*/

// fn a_func() -> &Box<u8> {
//     let x = Box::new(10);
//     &x
// }
/*
    Compiling playground v0.0.1 (/playground)
    error[E0106]: missing lifetime specifier
    --> src/main.rs:5:16
    |
    5 | fn a_func() -> &Box<u8> {
    |                ^ expected named lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    help: consider using the `'static` lifetime
    |
    5 | fn a_func() -> &'static Box<u8> {
    |                ^^^^^^^^
*/

fn a_func() -> Box<u8> {
    let x = Box::new(10);
    x
}