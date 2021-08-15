use num::Num;
use thiserror::Error;

#[derive(Error, Debug)]
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
/// The cardinality of the slice of constants must equal the degree of the polynomial.
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
pub fn eval_polynomial<T: Num + Copy> (x: T, constants: &[T]) -> Result<T, PolynomialEvalError>
{
  let (&k, constants) = constants.split_last().ok_or(PolynomialEvalError::CardinalityTooLow)?;
  if constants.len() >= 1 {
    Ok(x * eval_polynomial(x, constants)? + k)
  } else {
    Ok(k)
  }
}