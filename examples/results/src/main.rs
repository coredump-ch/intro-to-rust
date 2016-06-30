fn main() {
    println!("Hello, {:?}!", do_calc());
}

fn do_calc() -> Result<i32, String> {
    let a = match do_subcalc1() {
        Ok(val) => val,
        Err(msg) => return Err(msg),
    };
    let b = match do_subcalc2() {
        Ok(val) => val,
        Err(msg) => return Err(msg),
    };
    Ok(a + b)
}

fn do_subcalc1() -> Result<i32, String> {
  Ok(64)
}
fn do_subcalc2() -> Result<i32, String> {
  Ok(42)
}
