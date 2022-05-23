mod bindings;

fn main() {
    println!("Hello, world from Rust!");
    let mut cs = bindings::CoolStruct { x: 15, y: 32 };
    unsafe {
        bindings::cool_function(12, 'g' as i8, &mut cs as *mut bindings::CoolStruct);
    }
}
