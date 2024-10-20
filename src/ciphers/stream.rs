/// RC4 (Rivest Cipher 4) ストリーム暗号の実装
pub struct RC4 {
    s: [u8; 256],  // S-box
    i: u8,         // インデックス i
    j: u8,         // インデックス j
}

impl RC4 {
    /// 新しいRC4インスタンスを作成する
    ///
    /// # 引数
    /// * `key` - 暗号化キー
    ///
    /// # 戻り値
    /// 初期化されたRC4構造体
    pub fn new(key: &[u8]) -> Self {
        let mut s = [0u8; 256];
        // S-boxの初期化
        for (i, item) in s.iter_mut().enumerate() {
            *item = i as u8;
        }

        // キーを使用してS-boxをシャッフル
        let mut j: u8 = 0;
        for i in 0..256 {
            j = j.wrapping_add(s[i]).wrapping_add(key[i % key.len()]);
            s.swap(i, j as usize);
        }

        RC4 { s, i: 0, j: 0 }
    }

    /// データを暗号化（または復号）する
    ///
    /// # 引数
    /// * `data` - 暗号化（または復号）するデータ
    pub fn encrypt(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            // i と j を更新
            self.i = self.i.wrapping_add(1);
            self.j = self.j.wrapping_add(self.s[self.i as usize]);
            
            // S-box内の値を交換
            self.s.swap(self.i as usize, self.j as usize);
            
            // キーストリームバイトを生成
            let k = self.s[(self.s[self.i as usize].wrapping_add(self.s[self.j as usize])) as usize];
            
            // データとキーストリームバイトをXOR
            *byte ^= k;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc4_encrypt() {
        let key = b"Key";
        let mut rc4 = RC4::new(key);
        let mut data = b"Hello, World!".to_vec();
        rc4.encrypt(&mut data);
        
        // 暗号化されたデータが元のデータと異なることを確認
        assert_ne!(data, b"Hello, World!");
    }

    #[test]
    fn test_rc4_decrypt() {
        let key = b"Key";
        let mut rc4 = RC4::new(key);
        let mut data = b"Hello, World!".to_vec();
        
        // 暗号化
        rc4.encrypt(&mut data);
        
        // 新しいRC4インスタンスで復号
        let mut rc4 = RC4::new(key);
        rc4.encrypt(&mut data);
        
        // 復号されたデータが元のデータと一致することを確認
        assert_eq!(data, b"Hello, World!");
    }

    #[test]
    fn test_rc4_different_keys() {
        let key1 = b"Key1";
        let key2 = b"Key2";
        let mut rc4_1 = RC4::new(key1);
        let mut rc4_2 = RC4::new(key2);
        
        let mut data1 = b"Hello, World!".to_vec();
        let mut data2 = b"Hello, World!".to_vec();
        
        rc4_1.encrypt(&mut data1);
        rc4_2.encrypt(&mut data2);
        
        // 異なるキーで暗号化されたデータが異なることを確認
        assert_ne!(data1, data2);
    }

    #[test]
    fn test_rc4_long_message() {
        let key = b"LongKey";
        let mut rc4 = RC4::new(key);
        let mut data = b"This is a longer message to test RC4 encryption with more data".to_vec();
        let original_data = data.clone();
        
        // 暗号化
        rc4.encrypt(&mut data);
        assert_ne!(data, original_data);
        
        // 復号
        let mut rc4 = RC4::new(key);
        rc4.encrypt(&mut data);
        assert_eq!(data, original_data);
    }
}
