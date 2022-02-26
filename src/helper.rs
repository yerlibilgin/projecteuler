use std::collections::HashSet;

pub(crate) fn is_prime(number: i64) -> bool {
  let sqrt_i = (number as f64).sqrt() as i64;

  for i in (3..=sqrt_i).step_by(2) {
    if number % i == 0 {
      return false;
    }
  }
  true
}

pub(crate) fn find_prime_factors(mut source: i64) -> HashSet<i64> {
  let source_f = source as f64;

  let value = source_f.sqrt() as i64;

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

#[cfg(test)]
mod tests {
  #[test]
  fn is_prime() {
    assert_eq!(crate::helper::is_prime(100), false);
    assert_eq!(crate::helper::is_prime(99), false);
    assert_eq!(crate::helper::is_prime(97), true);
    assert_eq!(crate::helper::is_prime(13), true);
    assert_eq!(crate::helper::is_prime(25), false);
    assert_eq!(crate::helper::is_prime(1000), false);
    assert_eq!(crate::helper::is_prime(1009), true);
  }
}