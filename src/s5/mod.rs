// 5. Longest Palindromic Substring

// mod dp;
// pub use dp::longest_palindrome;

// mod center;
// pub use center::longest_palindrome;

mod manacher;
pub use manacher::longest_palindrome;

#[cfg(test)]
mod tests;
