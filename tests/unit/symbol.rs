use compiler_tools::symbol::Symbols;

#[test]
fn test_symbol_equality() {
    let hello_a = "helloa";
    let hello_b = "hellob";
    let mut gensym = Symbols::new();
    let a = gensym.symbol_nonnull(&hello_a.split_at(5).0);
    let b = gensym.symbol_nonnull(&hello_b.split_at(5).0);

    assert_eq!(a, b)
}

#[test]
fn test_symbol_non_equality() {
    let hello_a = "helloa";
    let hello_b = "hellob";
    let mut gensym = Symbols::new();
    let a = gensym.symbol_nonnull(&hello_a);
    let b = gensym.symbol_nonnull(&hello_b);

    assert_ne!(a, b)
}

#[test]
fn test_eq_str() {
    let hello = "hello";
    let mut gensym = Symbols::new();
    let sym = gensym.symbol_nonnull(&hello);

    assert_eq!(&sym, "hello");
    assert_eq!("hello", &sym)
}

#[test]
fn test_ne_str() {
    let hello = "helloa";
    let mut gensym = Symbols::new();
    let sym = gensym.symbol_nonnull(&hello);

    assert_ne!(&sym, "hellob");
    assert_ne!("hellob", &sym)
}

#[test]
fn test_ord_str() {
    let hello = "helloa";
    let mut gensym = Symbols::new();
    let sym = gensym.symbol_nonnull(&hello);

    assert!(&sym < "hellob");
    assert!("hellob" > &sym)
}
