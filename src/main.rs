fn main() {
  println!("Hello, world!");
}

use std::collections::HashSet;

mod helper {
  use std::collections::HashSet;

  pub(crate) fn is_prime(number: i64) -> bool {
    let sqrt_i = (number as f64).sqrt() as i64;

    for i in (3..=sqrt_i).step_by(2) {
      if number % i == 0 {
        println!("{}", i);
        return false;
      }
    }

    true
  }

pub(crate) fn find_prime_factors(mut source: i64) -> HashSet<i64> {
  let source_f = source as f64;

  let mut value = source_f.sqrt() as i64;

  let mut set = HashSet::<i64>::new();
  if source % 2 == 0 {
    set.insert(2);
    source = source / 2;
    while source % 2 == 0 {
      source = source / 2;
    }
  }

  for val in (3..value).step_by(2) {
    if source % val == 0 {
      set.insert(val);
      source = source / val;
      while source % val == 0 {
        source = source / val;
      }
    }
  }

  set
}
}


#[cfg(test)]
mod test {
  use std::collections::HashSet;
  use std::iter;
  use std::iter::repeat_with;
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
    let mut source = 600851475143i64;

    let set = find_prime_factors(source);

    println!("factors:{:#?}", set)
  }

  #[test]
  fn p_0786() {}


  #[test]
  fn is_prime() {
    assert_eq!(crate::helper::is_prime(100), false);
    assert_eq!(crate::helper::is_prime(99), false);
    assert_eq!(crate::helper::is_prime(97), true);
    assert_eq!(crate::helper::is_prime(13), true);
    assert_eq!(crate::helper::is_prime(25), false);
    assert_eq!(crate::helper::is_prime(1000), false);
    assert_eq!(crate::helper::is_prime(1007), true);
  }
}