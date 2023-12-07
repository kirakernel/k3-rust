fn main() {
    // loop_one();
    // loop_one_break_expression();
    // loop_two();
    // _loop_two_while_loop_print_elms();
    //  the_great_for_loop_part_one();
    the_great_for_loop_part_two();
}

fn _loop_one() {
    loop {
        println!("again!");
    }
}

fn __loop_one_break_expression() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn _loop_two_while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1
    }

    println!("LIFTOFF!!!!");
}

fn _loop_two_while_loop_print_elms() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn _the_great_for_loop_part_one() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}")
    }
}

fn the_great_for_loop_part_two() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!!");
}
