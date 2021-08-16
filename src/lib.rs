use num_traits::ops::mul_add::MulAddAssign;
use num_traits::Zero;

/// Evaluate a polynomial of arbitrary rank using Horner's method.
///
/// Horner's method goes like this: to find 𝑎𝑥³+𝑏𝑥²+𝑐𝑥+𝑑, you evaluate 𝑥(𝑥(𝑎𝑥+𝑏)+𝑐)+𝑑.
///
/// That's what this function does too.
///
/// You provide a value for `𝑥` and a slice of values for the coefficients `&[𝑎, 𝑏, 𝑐, 𝑑, …]`.
/// The cardinality of the slice of coefficients must equal the degree of the polynomial plus one,
/// except for the special case of the whole expression being just 0 in which case a slice of
/// length zero means the same (gives you the same result) as if the slice was equal to `&[0]`
/// or any other number of all zeros.
///
/// Here are some examples demonstrating use of eval_polynomial:
///
/// ```
/// use horner::eval_any_rank_polynomial;
///
/// // Evaluating the polynomial 72𝑥²+81𝑥+99 with 𝑥 = 5
/// let val = eval_any_rank_polynomial(5, &[72, 81, 99]);
///
/// // Traditional calculation.
/// let trad = 72 * 5_i32.pow(2) + 81 * 5 + 99;
///
/// assert_eq!(val, trad);
/// ```
///
/// ```
/// # use horner::eval_any_rank_polynomial;
/// // Here we have the "polynomial" 42, which is to say, 42𝑥⁰. Evaluated with 𝑥 = 9000
/// assert_eq!(42, eval_any_rank_polynomial(9000, &[42]));
/// ```
///
/// ```
/// # use horner::eval_any_rank_polynomial;
/// // 23𝑥⁹+0𝑥⁸+27𝑥⁷+0𝑥⁶-5𝑥⁵+0𝑥⁴+0𝑥³+0𝑥²+0𝑥ⁱ+0𝑥⁰
/// // Written simply: 23𝑥⁹+27𝑥⁷-5𝑥⁵
/// // Evaluated with 𝑥 = 99
///
/// let val = eval_any_rank_polynomial(99_i128, &[23, 0, 27, 0, -5, 0, 0, 0, 0, 0]);
/// let trad = 23 * 99_i128.pow(9) + 27 * 99_i128.pow(7) - 5 * 99_i128.pow(5);
///
/// assert_eq!(val, trad);
/// ```
///
/// ```
/// # use horner::eval_any_rank_polynomial;
/// // The "polynomial" 0. Represented in it's simplest form, the list of coefficents is empty.
/// // Evaluated with 𝑥 = 222
/// assert_eq!(0, eval_any_rank_polynomial(222, &[]));
/// ```
///
/// See also: [eval_known_rank_polynomial]
pub fn eval_any_rank_polynomial<T: Zero + MulAddAssign + Copy> (x: T, coefficients: &[T]) -> T
{
  if let Some((&k, coefficients)) = coefficients.split_first() {
    let mut val = k;
    let mut it = coefficients.iter();
    while let Some(&k) = it.next() {
      val.mul_add_assign(x, k);
    }
    val
  } else {
    T::zero()
  }
}

/// Evaluate a polynomial of rank known at compile-time using Horner's method.
///
/// For now this function simply calls [eval_any_rank_polynomial], but the idea
/// is that in the future we may be able to optimize our code further in the case
/// where the rank of the polynomial is known at compile-time.
///
/// Example usage:
///
/// ```
/// use horner::eval_known_rank_polynomial;
///
/// assert_eq!(0, eval_known_rank_polynomial(-4, &[1, 4]));
/// ```
///
/// See also: [eval_any_rank_polynomial]
pub fn eval_known_rank_polynomial<T: Zero + MulAddAssign + Copy, const N: usize> (x: T, coefficients: &[T; N]) -> T
{
  eval_any_rank_polynomial(x, coefficients)
}
