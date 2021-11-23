/// #9 回文数
/// 判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
///
/// 示例 1:
///
/// 输入: 121
/// 输出: true
/// 示例 2:
///
/// 输入: -121
/// 输出: false
/// 解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
/// 示例 3:
///
/// 输入: 10
/// 输出: false
/// 解释: 从右向左读, 为 01 。因此它不是一个回文数。
/// 进阶:
///
/// 你能不将整数转为字符串来解决这个问题吗？
///
#[allow(dead_code)]
fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut reverted_number = 0;
    let mut x = x;
    while x > reverted_number {
        reverted_number = reverted_number * 10 + x % 10;
        x /= 10;
    }
    x == reverted_number || x == reverted_number / 10
}

#[test]
fn is_palindrome_test() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(12321), true);
    assert_eq!(is_palindrome(123), false);
    assert_eq!(is_palindrome(-123), false);
    assert_eq!(is_palindrome(0), true);
}
