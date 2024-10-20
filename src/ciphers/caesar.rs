// カエサル暗号の実装

/// カエサル暗号で文字列を暗号化する関数
///
/// # 引数
/// * `plaintext` - 暗号化する文字列
/// * `shift` - シフトする文字数（0-25）
///
/// # 戻り値
/// 暗号化された文字列
pub fn caesar_encrypt(plaintext: &str, shift: u8) -> String {
    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                // アルファベットの文字のみを暗号化
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // 文字をシフトし、必要に応じてアルファベット内で巡回させる
                ((c as u8 - base + shift) % 26 + base) as char
            } else {
                // アルファベット以外の文字はそのまま
                c
            }
        })
        .collect()
}

/// カエサル暗号で文字列を復号する関数
///
/// # 引数
/// * `ciphertext` - 復号する暗号文
/// * `shift` - シフトする文字数（0-25）
///
/// # 戻り値
/// 復号された文字列
pub fn caesar_decrypt(ciphertext: &str, shift: u8) -> String {
    // 復号は、26からシフト値を引いた値で暗号化することと同じ
    caesar_encrypt(ciphertext, 26 - shift)
}

// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_encrypt() {
        assert_eq!(caesar_encrypt("hello", 3), "khoor");
        assert_eq!(caesar_encrypt("WORLD", 3), "ZRUOG");
        assert_eq!(caesar_encrypt("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn test_caesar_decrypt() {
        assert_eq!(caesar_decrypt("khoor", 3), "hello");
        assert_eq!(caesar_decrypt("ZRUOG", 3), "WORLD");
        assert_eq!(caesar_decrypt("Khoor, Zruog!", 3), "Hello, World!");
    }

    #[test]
    fn test_caesar_roundtrip() {
        let original = "The quick brown fox jumps over the lazy dog";
        let shift = 7;
        let encrypted = caesar_encrypt(original, shift);
        let decrypted = caesar_decrypt(&encrypted, shift);
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_caesar_with_non_alphabetic() {
        assert_eq!(caesar_encrypt("a1b2c3!", 1), "b1c2d3!");
        assert_eq!(caesar_decrypt("b1c2d3!", 1), "a1b2c3!");
    }

    #[test]
    fn test_caesar_wrap_around() {
        assert_eq!(caesar_encrypt("xyz", 3), "abc");
        assert_eq!(caesar_decrypt("abc", 3), "xyz");
    }
}
