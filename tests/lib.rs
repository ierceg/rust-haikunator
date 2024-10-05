use haikunator::{Haikunator, HaikunatorParams};
use regex::Regex;
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[test]
fn it_returns_4_digits_token() {
    let h = Haikunator::default();
    let name = h.haikunate();

    let re = Regex::new(r"^\w+-\w+-[0123456789]{4}$").unwrap();

    assert!(re.is_match(&name));
}

#[test]
fn it_does_not_return_the_same_name() {
    let h = Haikunator::default();
    let name1 = h.haikunate();
    let name2 = h.haikunate();

    assert!(!(&name1 == &name2));
}

#[test]
fn it_uses_custom_delimiter() {
    let mut h = Haikunator::default();
    h.delimiter = "@";
    let name = h.haikunate();

    let re = Regex::new(r"^\w+@\w+@[0123456789]{4}$").unwrap();

    assert!(re.is_match(&name));
}

#[test]
fn it_returns_4_digits_hex_token() {
    let h = Haikunator::new(HaikunatorParams {
        delimiter: "-",
        token_length: 4,
        token_hex: true,
        token_chars: "overriden by token_hex=true",
        ..Default::default()
    });

    let re = Regex::new(r"^\w+-\w+-[0123456789abcdef]{4}$").unwrap();

    for _ in 0..100 {
        assert!(re.is_match(&h.haikunate()));
    }
}

#[test]
fn it_uses_custom_adj_noun() {
    let h = Haikunator::new(HaikunatorParams {
        adjectives: &["flying", "bubbly"],
        nouns: &["bat", "soda"],
        delimiter: "-",
        token_length: 4,
        token_hex: false,
        token_chars: "123",
        ..Default::default()
    });

    let name = h.haikunate();
    let parts: Vec<&str> = name.split("-").collect();

    assert!(parts[0] == "flying" || parts[0] == "bubbly");
    assert!(parts[1] == "bat" || parts[1] == "soda");
}

#[test]
fn it_returns_10_count_tokens() {
    let h = Haikunator::new(HaikunatorParams {
        adjectives: &["flying", "bubbly"],
        nouns: &["bat", "soda"],
        delimiter: "-",
        token_length: 10,
        token_hex: false,
        token_chars: "0123456789忠犬ハチ公",
        ..Default::default()
    });

    let re = Regex::new(r"^\w+-\w+-[0123456789忠犬ハチ公]{10}$").unwrap();

    for _ in 0..100 {
        assert!(re.is_match(&h.haikunate()));
    }
}

#[test]
fn it_drops_token_if_length_is_0() {
    let mut h = Haikunator::default();
    h.token_length = 0;

    let re = Regex::new(r"^\w+-\w+$").unwrap();

    for _ in 0..100 {
        assert!(re.is_match(&h.haikunate()));
    }
}

#[test]
fn it_permits_custom_token_chars() {
    let mut h = Haikunator::default();
    h.token_chars = "foo";

    let re = Regex::new(r"^\w+-\w+-[foo]{4}$").unwrap();

    for _ in 0..100 {
        let name = &h.haikunate();
        assert!(re.is_match(name));
    }
}

#[test]
fn it_supports_unicode_codepoints() {
    let h = Haikunator::new(HaikunatorParams {
        delimiter: "-",
        token_length: 5,
        token_hex: false,
        token_chars: "忠犬ハチ公",
        ..Default::default()
    });

    let re = Regex::new(r"^\w+-\w+-[0123456789忠犬ハチ公]{5}$").unwrap();

    for _ in 0..100 {
        assert!(re.is_match(&h.haikunate()));
        // ^ would fail if we instead used
        // `let count = tokens.len();`
        // in haikunate().
    }
}

#[test]
fn it_handles_zero_length_parts_without_gen_range_panic() {
    let mut h = Haikunator::default();
    h.token_length = 0;
    h.adjectives = &[];
    h.nouns = &[];
    h.haikunate(); // no panic
}

#[test]
fn it_works_with_small_rng() {
    let params = HaikunatorParams {
        rng: SmallRng::seed_from_u64(42),
        adjectives: &["swift", "silent"],
        nouns: &["fox", "dog"],
        delimiter: "_",
        token_length: 4,
        token_hex: false,
        token_chars: "0123456789",
    };

    let h = Haikunator::new(params);

    // Generate a few haikunates and check if they're consistent
    let result1 = h.haikunate();
    let result2 = h.haikunate();
    let result3 = h.haikunate();

    assert_ne!(result1, result2);
    assert_ne!(result2, result3);

    // Check the format of the generated name
    let parts: Vec<&str> = result1.split('_').collect();
    assert_eq!(parts.len(), 3);
    assert!(parts[0] == "swift" || parts[0] == "silent");
    assert!(parts[1] == "fox" || parts[1] == "dog");
    assert_eq!(parts[2].len(), 4);
    assert!(parts[2].chars().all(|c| c.is_digit(10)));
}
