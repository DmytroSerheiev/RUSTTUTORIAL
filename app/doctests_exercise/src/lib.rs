
//! Вправа - Напишіть тести з документації
/// This function divides two numbers.
///
/// # Example #1: 10 / 2 == 5
///
/// ```
/// use doctests_exercise::div;
/// let result = div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Example #2: 6 / 3 == 2
///
/// ```
/// use doctests_exercise::div;
/// let result = div(6, 3);
/// assert_eq!(result, 2);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// use doctests_exercise::div;
/// assert!(std::panic::catch_unwind(|| div(10, 0)).is_err());
/// ```

/// This function subtracts two numbers.
///
/// # Example #1: 9 - 2 == 7
///
/// ```
/// use doctests_exercise::sub;
/// let result = sub(9, 2);
/// assert_eq!(result, 7);
/// ```
///
/// # Example #2: 6 - 9 == -3
///
/// ```
/// use doctests_exercise::sub;
/// let result = sub(6, 9);
/// assert_eq!(result, -3);
/// ```
