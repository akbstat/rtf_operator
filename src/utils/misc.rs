/// find out the position of specify pattern in occurs for the first time
///
/// @pattern: the pattern you want to find out
///
/// @source: target that you want to find out group
///
/// @pointer: the start position in the source
///
/// ```rust
/// #[test]
/// fn test_pattern_position() {
///     let content = br"{\fonttbl{\f1\froman\fprq2\fcharset0 SimSun;}".to_vec();
///     let pattern = br"\fonttbl".to_vec();
///     let result = pattern_position(&pattern, &content, 0).unwrap();
///     let result = String::from_utf8(content.get(result.0..result.1).unwrap().into()).unwrap();
///     assert_eq!(pattern, result.as_bytes());
///     let pattern = br"\test".to_vec();
///     let result = pattern_position(&pattern, &content, 0);
///     assert_eq!(result, None);
/// }
/// ```
pub fn pattern_position(pattern: &[u8], source: &[u8], pointer: usize) -> Option<(usize, usize)> {
    let mut pointer = pointer;
    let pattern_size = pattern.len();
    if pointer > source.len() {
        return None;
    }
    while pointer < source.len() {
        if pointer < pattern_size + 1 {
            pointer += 1;
            continue;
        }
        let start = pointer - pattern_size - 1;
        let end = pointer - 1;
        match source.get(start..end) {
            Some(target) => {
                if pattern.eq(target) {
                    return Some((start, end));
                }
            }
            None => {
                continue;
            }
        };
        pointer += 1;
    }
    None
}
