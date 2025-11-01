fn call_fn_once<F: FnOnce()>(f: F) { f(); }
fn call_fn_mut<F: FnMut()>(mut f: F) { f(); }
fn call_fn<F: Fn()>(f: F) { f(); }

fn main() {
    let name = String::from("Rust");

    let say_hi = || println!("Hi, {}", name); // Fn
    call_fn(say_hi);      // ok
    call_fn_mut(say_hi);  // ok — Fn is also FnMut
    call_fn_once(say_hi); // ok — Fn is also FnOnce
}
