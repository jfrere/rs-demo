#![allow(dead_code, unused_variables, unused_mut)]

fn moving_data() {
    // * * * * *
    // * (1) assigning to new variables
    // * * * * *
    let string = "my string data".to_string();
    let new_string = string;
    println!("{:?}", new_string);
    // println!("{:?}", my_string);

    // * * * * *
    // * (2) passing to functions
    // * * * * *
    fn print_to_upper(string: String) {
        println!("{:?}", string.to_uppercase());
    }

    let string = "my string data".to_string();
    print_to_upper(string);
    // println!("{:?}", string);

    // * * * * *
    // * (3) returning from functions
    // * * * * *
    fn return_to_upper(string: String) -> String {
        string.to_uppercase()
    }

    let string = "my string data".to_string();
    let new_string = return_to_upper(string);
}

fn dropping_data() {
    // data is dropped when the variable that owns the data goes out of scope
    {
        let string = "hello".to_string();

        // <-- the data is dropped here
    }

    fn function_to_drop_data(argument: String) {
        let function_local_variable = "hello".to_string();

        // <-- both `argument` and `function_local_variable` are dropped here
    }

    let string = "hello".to_string();
    drop(string); // Q: what is the source code of the drop function?
}

fn immutable_borrow() {
    let mut string = "my string data".to_string();
    {
        let reference = &string;
        println!("{:?} {:?}", string, reference);

        // string.clear();
        // reference.clear();
        // drop(string);
        // drop(reference);

        fn read_string_reference(string: &String) {
            println!("{:?}", string);
        }

        read_string_reference(&string);
        read_string_reference(reference);
        println!("{:?} {:?} {:?}", string, reference, &string);

        println!("{:?}", reference);
    }

    string.clear();
    drop(string);
}

fn mutable_borrow() {
    let mut string = "my string data".to_string();
    {
        let reference = &mut string;
        println!("{:?}", reference);
        reference.clear();

        // println!("{:?}", string);
        // let immutable_reference = &string;
        // string.clear();
        // drop(string);
        // drop(reference);

        let immutable_reference = &*reference;
        println!("{:?}", immutable_reference);

        fn write_string_reference(string: &mut String) {
            println!("{:?}", string);
        }

        write_string_reference(reference);

        println!("{:?}", reference);
    }

    println!("{:?}", string);
    string.clear();
    drop(string);
}

fn main() {}
