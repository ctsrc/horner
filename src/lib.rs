use num_traits::ops::mul_add::MulAddAssign;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PolynomialEvalError {
  #[error("Cardinality of constants slice is too low.")]
  CardinalityTooLow,
}

/// Evaluate a polynomial using Horner's method.
///
/// Horner's method goes like this: to find 𝑎𝑥³+𝑏𝑥²+𝑐𝑥+𝑑, you evaluate 𝑥(𝑥(𝑎+𝑏)+𝑐)+𝑑.
///
/// That's what this function does too.
///
/// You provide a value for `𝑥` and a slice of values for the constants `&[𝑎, 𝑏, 𝑐, 𝑑, ]`.
/// The cardinality of the slice of constants must equal the degree of the polynomial plus one.
///
/// Here are some examples demonstrating use of eval_polynomial:
///
/// ```
/// use horner::eval_polynomial;
///
/// // Evaluating the polynomial 72𝑥²+81𝑥+99 with 𝑥 = 5
/// let val = eval_polynomial(5, &[72, 81, 99]).unwrap();
///
/// // Equivalent calculation "by hand", so to speak.
/// let equiv = 72 * 5_i32.pow(2) + 81 * 5 + 99;
///
/// assert_eq!(val, equiv);
/// ```
///
/// ```
/// # use horner::eval_polynomial;
/// // Here we have the "polynomial" 42, which is to say, 42𝑥⁰ if you will.
/// assert_eq!(Ok(42), eval_polynomial(9000, &[42]));
/// ```
pub fn eval_polynomial<T: MulAddAssign + Copy> (x: T, constants: &[T]) -> Result<T, PolynomialEvalError>
{
  let (&k, constants) = constants.split_first().ok_or(PolynomialEvalError::CardinalityTooLow)?;
  Ok(eval_polynomial_inner(x, constants, k))
}

fn eval_polynomial_inner<T: MulAddAssign + Copy> (x: T, constants: &[T], n: T) -> T
{
  let mut result = n;
  let mut it = constants.iter();
  while let Some(&k) = it.next() {
    result.mul_add_assign(x, k);
  }
  result
}
