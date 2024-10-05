use rand::Rng;
use std::cell::RefCell;

mod default_adjectives;
mod default_nouns;

/// The `Haikunator` type
/// Holds settings and data that will be used when `haikunate` is called.
///
/// # Examples
///
/// ```
/// use haikunator::{Haikunator, HaikunatorParams};
///
/// let h = Haikunator::new(HaikunatorParams {
///     rng: rand::rngs::ThreadRng::default(),
///     adjectives: &["flying", "bubbly"],
///     nouns: &["bat", "soda"],
///     delimiter: "-",
///     token_length: 8,
///     token_hex: false,
///     token_chars: "0123456789忠犬ハチ公"
/// });
///
/// ```
///
/// **Note**: If `token_hex` is true, the value of `token_chars` is ignored.
#[derive(Debug)]
pub struct Haikunator<'a, R: Rng> {
    rng: RefCell<R>,
    pub adjectives: &'a [&'a str],
    pub nouns: &'a [&'a str],
    pub delimiter: &'a str,
    pub token_length: usize,
    pub token_hex: bool,
    pub token_chars: &'a str,
}

/// Parameters for `Haikunator::new_parametrized`.
pub struct HaikunatorParams<'a, R: Rng> {
    pub rng: R,
    pub adjectives: &'a [&'a str],
    pub nouns: &'a [&'a str],
    pub delimiter: &'static str,
    pub token_length: usize,
    pub token_hex: bool,
    pub token_chars: &'static str,
}

impl Default for HaikunatorParams<'static, rand::rngs::ThreadRng> {
    fn default() -> Self {
        Self {
            rng: rand::rngs::ThreadRng::default(),
            adjectives: default_adjectives::DEFAULT_ADJECTIVES,
            nouns: default_nouns::DEFAULT_NOUNS,
            delimiter: "-",
            token_length: 4,
            token_hex: false,
            token_chars: "0123456789",
        }
    }
}

impl<'a, R: Rng> Haikunator<'a, R> {
    /// Creates a new Haikunator with the given parameters.
    pub fn new(params: HaikunatorParams<'a, R>) -> Self {
        Self {
            rng: RefCell::new(params.rng),
            adjectives: params.adjectives,
            nouns: params.nouns,
            delimiter: params.delimiter,
            token_length: params.token_length,
            token_hex: params.token_hex,
            token_chars: params.token_chars,
        }
    }

    /// Generates random heroku-like short names using a combination
    // of adjective, noun, and the delimiter.
    ///
    /// # Examples
    ///
    /// ```
    /// use haikunator::Haikunator;
    ///
    /// let h = Haikunator::default();
    /// println!("{:?}", h.haikunate());
    /// ```
    pub fn haikunate(&self) -> String {
        let tokens = if self.token_hex {
            "0123456789abcdef"
        } else {
            self.token_chars
        };

        let mut rng = self.rng.borrow_mut();
        let adjective = if !self.adjectives.is_empty() {
            self.adjectives[rng.gen_range(0..self.adjectives.len())]
        } else {
            ""
        };

        let noun = if !self.nouns.is_empty() {
            self.nouns[rng.gen_range(0..self.nouns.len())]
        } else {
            ""
        };

        let mut token = String::with_capacity(self.token_length);
        let count = tokens.chars().count();

        if count > 0 {
            for _ in 0..self.token_length {
                let index = rng.gen_range(0..count);
                token.push(tokens.chars().nth(index).unwrap());
            }
        }

        let mut parts = vec![adjective, noun, &token];
        parts.retain(|s: &&str| !s.is_empty());
        parts.join(self.delimiter)
    }
}

impl Default for Haikunator<'static, rand::rngs::ThreadRng> {
    /// Constructs a new Haikunator with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use haikunator::Haikunator;
    ///
    /// let h = Haikunator::default();
    /// ```
    fn default() -> Self {
        Self::new(HaikunatorParams::default())
    }
}
