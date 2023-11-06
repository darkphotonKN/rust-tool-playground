
/// This is a sum function that adds to numbers
/// 
/// # Arguments 
/// sum(num1, num2)
/// num1: u32
/// num2: u32
/// 
/// # Examples
/// ```
/// let result = my_funcs::sum(1, 2);
/// println!("result: {}", result);
/// ```

// pub to allow other modules to use this function
pub fn sum(num1: u32, num2: u32) -> u32 {
  let result: u32 = num1 + num2;
  result
}



// unit test for sum function
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sum() {
    assert_eq!(sum(1, 2), 3);
  }
}