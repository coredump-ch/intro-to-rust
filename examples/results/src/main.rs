fn main() {
    println!("match     {:?}!", do_calc_match());
    println!("try       {:?}!", do_calc_try());
    println!("operator? {:?}!", do_calc_operator());
}

fn do_calc_match() -> Result<i32, String> {
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

fn do_calc_try() -> Result<i32, String> {
    let a = try!( do_subcalc1() );
    let b = try!( do_subcalc2() );
    Ok(a + b)
}

fn do_calc_operator() -> Result<i32, String> {
    let a = do_subcalc1()?;
    let b = do_subcalc2()?;
    Ok(a + b)
}

fn do_subcalc1() -> Result<i32, String> {
  Ok(64)
}
fn do_subcalc2() -> Result<i32, String> {
  Ok(42)
}
