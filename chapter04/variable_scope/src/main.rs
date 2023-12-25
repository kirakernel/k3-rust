fn main() {
    {
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
        println!("{}, world!", s);
    } // s is not valid here, it's not yet declared
}
