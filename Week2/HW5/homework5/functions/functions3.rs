// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    call_this();
}

fn call_this(num: i32) {
    for i in 0..num {
        println!("Loop now {}", i + 1);
    }
}
