#[cfg(test)]
mod test {
  use std::iter;
  use crate::helper::find_prime_factors;

  ///
  /// Multiples of 3 or 5
  ///
  #[test]
  fn p_0001() {
    println!("Sum: {}",
             (1..1000).filter(|x| (x % 5 == 0 || x % 3 == 0))
               .fold(0, |sum, x| sum + x)
    )
  }


  #[test]
  fn p_0002() {
    let mut curr = 2i64;
    let mut prev = 1i64;
    let sum = iter::repeat_with(|| {
      (curr, prev) = (prev + curr, curr);
      curr
    }).take_while(|x| *x < 4000000)
      .filter(|x| x % 2 == 0)
      .fold(2i64, |sum, even| sum + even);

    println!("Fib {}", sum);
  }

  #[test]
  fn p_0003() {

    //find prime factor of 600851475143
    let source = 600851475143i64;

    let set = find_prime_factors(source);

    println!("factors:{:#?}", set)
  }
}
