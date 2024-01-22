use std::time::Instant;
const PI: i8 = 3;

fn main() {
    num_types();
    fun_with_params(4);
    let y = statements();
    call_func(y, &fun_with_params);
    control_flow();
    loops(3);
    loop_label();
    whiles(76);
    fors();
    //fibonacci(0);
    //fibonacci(1);
    //fibonacci(2);
    //fibonacci(10);
    //fibonacci(19);
    //fibonacci(40);
    //fibonacci(94);

    //let now = Instant::now();
    //{
    //    fibonacci(40);
    //}
    //println!("\tSumatory {:.2?}", now.elapsed());
    println!("\nTesting fib as sum vs recursive: 45th number");

    println!("n,rust_sum");
    compute_fib_time(50, &fibonacci);
    println!("n,rust_recursive");
    compute_fib_time(50, &fibonacci_rec_v2);

    let now = Instant::now();
    {
        fibonacci(40);
    }
    println!("\tSumatory {:.2?}", now.elapsed());
    let now = Instant::now();
    {
        fibonacci_rec(40);
    }
    println!("\tRecursive {:.2?}", now.elapsed());
    let now = Instant::now();
    {
        fibonacci_rec_v2(40);
    }
    println!("\tRecursiveV2 {:.2?}", now.elapsed());
}

fn num_types() {
    let big_number: u128 = 92_923_233_239_233;
    let big_number: f64 = big_number as f64;
    let float: f64 = 2.2383;
    let division = big_number * float / (PI as f64);
    println!("Division is {division}");

    let int_division = 21 / 5;
    println!("Integer division is the default. Take care: {int_division}");
}

fn fun_with_params(x: i32) {
    println!("Param is {x}");
}

fn statements() -> i32 {
    let x = 1;
    let y = {
        let x = 324;
        x + 1
    };
    println!("x = {x} and y = {y}");
    y
}

fn call_func(x: i32, f: &dyn Fn(i32) -> ()) {
    // I still don't quite understand what dyn means. That it accepts any function with those
    // types?
    f(x);
}

fn control_flow() {
    let number = 2;

    if number < 3 {
        println!("Number lower than 3");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("Number is bigger than 3 and not divisible by it");
    }

    let oneline_condition = if true { 3 } else { 4 };
    println!("One line condition: {oneline_condition}");
}

fn loops(times: u32) {
    println!("Looping a maximum of {times} times");
    let mut i = 1;
    let result = loop {
        println!("\tLoop number {i}");
        if i >= times {
            break i * 2;
        }
        i += 1;
    };
    println!("Resulting number (because of plot is): {result}");
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn whiles(max_num: u32) {
    let mut start = 0;

    while max_num != start {
        start += 1;
    }
    println!("Done!");
}

fn fors() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn fibonacci(number: u32) -> u128 {

    let mut minus_one = 1;
    let mut minus_two = 0;
    let mut current = 0;

    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    }
    for _ in 2..=number {
        current = minus_two + minus_one;
        
        minus_two = minus_one;
        minus_one = current;
    }
    //println!("The {number} number in Fib's sequence is {current}");
    current
}

fn fibonacci_rec(number: u32) -> u128 {
    if number < 2 {
        number as u128
    } else {
        fibonacci_rec(number-1) + fibonacci_rec(number-2)
    }
}

fn fibonacci_rec_v2(number: u32) -> u128 {
    match number {
        1 | 0 => number as u128,
        _ => fibonacci_rec_v2(number - 1) + fibonacci_rec_v2(number - 2),
    }
}


fn compute_fib_time(max_num: u32, f: &dyn Fn(u32) -> u128) {
    for n in 1..=max_num {

        let now = Instant::now();
        {
            f(n);
        }
        //println!("{},{:.2?}", n, now.elapsed());
        let elapsed = now.elapsed();
        println!("{},{},{:.2?}", n, elapsed.as_nanos(), elapsed);
    }
}
