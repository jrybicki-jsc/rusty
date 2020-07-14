static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 7;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1={}", *r1);
        println!("r2={}", *r2);
    }

    add_to_count(3);
    unsafe {
        println!("COUNTER {}", COUNTER);
    }
}
