use num_traits::ops::mul_add::MulAddAssign;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PolynomialEvalError {
  #[error("Cardinality of coefficients slice is too low.")]
  CardinalityTooLow,
}

/// Evaluate a polynomial using Horner's method.
///
/// Horner's method goes like this: to find 𝑎𝑥³+𝑏𝑥²+𝑐𝑥+𝑑, you evaluate 𝑥(𝑥(𝑎𝑥+𝑏)+𝑐)+𝑑.
///
/// That's what this function does too.
///
/// You provide a value for `𝑥` and a slice of values for the coefficients `&[𝑎, 𝑏, 𝑐, 𝑑, …]`.
/// The cardinality of the slice of coefficients must equal the degree of the polynomial plus one.
///
/// Here are some examples demonstrating use of eval_polynomial:
///
/// ```
/// use horner::try_eval_polynomial;
///
/// // Evaluating the polynomial 72𝑥²+81𝑥+99 with 𝑥 = 5
/// let val = try_eval_polynomial(5, &[72, 81, 99]).unwrap();
///
/// // Traditional calculation.
/// let trad = 72 * 5_i32.pow(2) + 81 * 5 + 99;
///
/// assert_eq!(val, trad);
/// ```
///
/// ```
/// # use horner::try_eval_polynomial;
/// // Here we have the "polynomial" 42, which is to say, 42𝑥⁰. Evaluated with 𝑥 = 9000
/// assert_eq!(Ok(42), try_eval_polynomial(9000, &[42]));
/// ```
///
/// ```
/// # use horner::try_eval_polynomial;
/// // 23𝑥⁹+0𝑥⁸+27𝑥⁷+0𝑥⁶-5𝑥⁵+0𝑥⁴+0𝑥³+0𝑥²+0𝑥ⁱ+0𝑥⁰
/// // Written simply: 23𝑥⁹+27𝑥⁷-5𝑥⁵
/// // Evaluated with 𝑥 = 99
///
/// let val = try_eval_polynomial(99_i128, &[23, 0, 27, 0, -5, 0, 0, 0, 0, 0]).unwrap();
/// let trad = 23 * 99_i128.pow(9) + 27 * 99_i128.pow(7) - 5 * 99_i128.pow(5);
///
/// assert_eq!(val, trad);
/// ```
pub fn try_eval_polynomial<T: MulAddAssign + Copy> (x: T, coefficients: &[T]) -> Result<T, PolynomialEvalError>
{
  let (&k, coefficients) = coefficients.split_first().ok_or(PolynomialEvalError::CardinalityTooLow)?;

  let mut val = k;
  let mut it = coefficients.iter();
  while let Some(&k) = it.next() {
    val.mul_add_assign(x, k);
  }

  Ok(val)
}
