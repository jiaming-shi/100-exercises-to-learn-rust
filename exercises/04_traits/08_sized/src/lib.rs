pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // std::mem::size_of::<str>();
    // error[E0277]: the size for values of type `str` cannot be known at compilation time
    // --> exercises/04_traits/08_sized/src/lib.rs:6:25
    // |
    // 6 |     std::mem::size_of::<str>();
    // |                         ^^^ doesn't have a size known at compile-time
    // |
    // = help: the trait `Sized` is not implemented for `str`
    // note: required by an implicit `Sized` bound in `std::mem::size_of`
    // --> /private/tmp/rust-20240518-7986-rmyaj5/rustc-1.78.0-src/library/core/src/mem/mod.rs:312:1

    // For more information about this error, try `rustc --explain E0277`.
    // error: could not compile `sized` (lib) due to 1 previous error
}
