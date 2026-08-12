//! Cleanroom Rust port of upstream Go source file: `list/enumerator.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Predefined enumerators for lists: Alphabet, Arabic, Roman, Bullet,
//! Asterisk, and Dash.
//! </public-docs>

use std::sync::Arc;

use crate::tree::Children;

/// Enumerator enumerates a list. Given a list of items and the index of the
/// current enumeration, it returns the prefix that should be displayed for the
/// current item.
///
/// For example, a simple Arabic numeral enumeration would be:
///
/// ```text
/// func Arabic(_ Items, i int) string {
///     return fmt.Sprintf("%d.", i+1)
/// }
/// ```
pub type Enumerator = Arc<dyn Fn(&dyn Children, usize) -> String>;

/// Indenter indents the children of a tree.
pub type Indenter = Arc<dyn Fn(&dyn Children, usize) -> String>;

/// <upstream-comment>Alphabet is the enumeration for alphabetical listing.
///
/// ```text
/// a. Foo
/// b. Bar
/// c. Baz
/// d. Qux.
/// ```</upstream-comment>
pub fn alphabet(_: &dyn Children, i: usize) -> String {
    const ABC_LEN: usize = 26;
    let i = i as i64;
    let abc = ABC_LEN as i64;
    if i >= abc * abc + abc {
        let c1 = (b'A' as i64 + i / abc / abc - 1) as u8 as char;
        let c2 = (b'A' as i64 + (i / abc) % abc - 1) as u8 as char;
        let c3 = (b'A' as i64 + i % abc) as u8 as char;
        format!("{}{}{}.", c1, c2, c3)
    } else if i >= abc {
        let c1 = (b'A' as i64 + i / abc - 1) as u8 as char;
        let c2 = (b'A' as i64 + i % abc) as u8 as char;
        format!("{}{}.", c1, c2)
    } else {
        let c = (b'A' as i64 + i % abc) as u8 as char;
        format!("{}.", c)
    }
}

/// <upstream-comment>Arabic is the enumeration for arabic numerals listing.
///
/// ```text
/// 1. Foo
/// 2. Bar
/// 3. Baz
/// 4. Qux.
/// ```</upstream-comment>
pub fn arabic(_: &dyn Children, i: usize) -> String {
    format!("{}.", i + 1)
}

/// <upstream-comment>Roman is the enumeration for roman numerals listing.
///
/// ```text
/// I. Foo
/// II. Bar
/// III. Baz
/// IV. Qux.
/// ```</upstream-comment>
pub fn roman(_: &dyn Children, i: usize) -> String {
    let roman = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];
    let arabic = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let mut i = i as i64;
    let mut result = String::new();
    for (v, value) in arabic.iter().enumerate() {
        let value = *value as i64;
        while i >= value - 1 {
            i -= value;
            result.push_str(roman[v]);
        }
    }
    result.push('.');
    result
}

/// <upstream-comment>Bullet is the enumeration for bullet listing.
///
/// ```text
/// • Foo
/// • Bar
/// • Baz
/// • Qux.
/// ```</upstream-comment>
pub fn bullet(_: &dyn Children, _: usize) -> String {
    "•".to_string()
}

/// <upstream-comment>Asterisk is an enumeration using asterisks.
///
/// ```text
/// * Foo
/// * Bar
/// * Baz
/// * Qux.
/// ```</upstream-comment>
pub fn asterisk(_: &dyn Children, _: usize) -> String {
    "*".to_string()
}

/// <upstream-comment>Dash is an enumeration using dashes.
///
/// ```text
/// - Foo
/// - Bar
/// - Baz
/// - Qux.
/// ```</upstream-comment>
pub fn dash(_: &dyn Children, _: usize) -> String {
    "-".to_string()
}
