const PI: i8 = 3;

fn main() {
    num_types();
    fun_with_params(4);
    let y = statements();
    call_func(2, &fun_with_params);
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
