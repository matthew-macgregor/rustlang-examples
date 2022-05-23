// The three rules of ~~Robotics~~ ownership:
//
//   - Each value in Rust has a variable that’s called its owner.
//   - There can only be one owner at a time.
//   - When the owner goes out of scope, the value will be dropped.

#![allow(dead_code)]

fn owner_one() -> bool {
    // This works because it's a string literal and is built into the
    // executable itself.
    let s = "owner";
    let t = s;
    dbg!(s);
    dbg!(t);
    true
}

fn move_one() -> bool {
    let s = String::from("owner");
    let t = s;

    // You can't do this because the ownership has moved to t and s has been dropped.
    // This prevents a double free error from happening.
    //
    // println!("S is no longer valid: {}", s);
    // error[E0382]: borrow of moved value: `s`
    // --> ownership\src\lib.rs:19:42
    // |
    // 17 |     let s = String::from("owner");
    // |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
    // 18 |     let t = s;
    // |             - value moved here
    // 19 |     println!("S is no longer valid: {}", s);

    // You can clone the value, which is legal but less performant.
    let s2 = t.clone();
    println!("S2 is a clone: t={}, s2={}", t, s2);
    true
}

fn copy_trait() -> bool {
    let x = 5;
    let y = x;

    // This is legal. integers have a known size at compile time and are stored on the stack.
    // These types are annotated with Copy to show that they are still valid after assignment.
    // This is similar to by value in other languages, and the data is bitwise copied on
    // assignment. A type with Copy cannot also be annotated with Drop.

    // Some types that are Copy:

    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    // Bools
    let mut a = true;
    let b = a;
    a = false;
    if a == b {
        return false;
    }

    // Floating point
    let mut a = 0.32;
    let b = a;
    a = 3.14;
    if a == b {
        return false;
    }

    // Character char
    let mut c = 'c';
    let cc = c;
    c = 'a';
    if c == cc {
        return false;
    }

    // Tuples with Copy types
    let t = (32, 42);
    let u = t;
    // Eq trait allows us to compare these directly
    if u == t {
        return true;
    }

    println!("x = {}, y = {}", x, y);
    return true;
}

fn ownership_vs_copy() -> bool {
    let s = String::from("hello");
    ovc_takes_ownership(s);
    let x = 5;
    ovc_makes_copy(x);
    println!("Outer: {}", x);
    // Can't compile, s was moved!
    // println!("Outer: {}", s);

    true
}

fn ovc_takes_ownership(sstr: String) {
    println!("{}", sstr);
}

fn ovc_makes_copy(sint: i32) {
    println!("{}", sint);
}

fn return_values_scope() -> bool {
    let s1 = rvs_gives_ownership();
    let s2 = String::from("hello");
    let s3 = rvs_takes_and_gives_back(s2);
    println!("{}, {}", s1, s3);
    // No longer valid, s2 moved into s3:
    // println!("{}", s2);
    true
}

fn rvs_gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // moved to the caller
}

fn rvs_takes_and_gives_back(s: String) -> String {
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn owner_one_test() {
        assert_eq!(owner_one(), true);
    }

    #[test]
    fn move_one_test() {
        assert_eq!(move_one(), true);
    }

    #[test]
    fn copy_trait_test() {
        assert_eq!(copy_trait(), true);
    }

    #[test]
    fn ownership_vs_copy_test() {
        assert_eq!(ownership_vs_copy(), true);
    }

    #[test]
    fn return_values_scope_test() {
        assert_eq!(return_values_scope(), true);
    }
}
