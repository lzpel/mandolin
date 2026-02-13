//! スキーマの重複排除キャッシュ
//!
//! テンプレートレンダリング中にインラインスキーマが発見されると、
//! schema_push()で登録し、schema_drain()でまとめて出力する。
//! 循環参照を防ぐため、2段階の登録方式をとる:
//!   1回目: (pointer, None) → キーのみ登録（重複検出用）
//!   2回目: (pointer, Some(content)) → コンテンツを設定
//!
//! Arc<Mutex<>>はminijinja::EnvironmentがSend+Syncを要求するため必要。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// スキーマ重複排除キャッシュ
/// テンプレートの関数としてクロージャにキャプチャされるためCloneが必要
#[derive(Clone)]
pub struct SchemaCache(Arc<Mutex<HashMap<String, Option<String>>>>);

impl SchemaCache {
    /// 空のキャッシュを生成する
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    /// スキーマを登録する
    /// 新規登録ならtrue、既に存在するならfalseを返す
    /// contentがNone→Some(v)に変わるときのみ値を更新する
    pub fn push(&self, pointer: &str, content: Option<&str>) -> bool {
        let mut map = self.0.lock().unwrap();
        if let Some(existing) = map.get(pointer) {
            // 既存エントリ: NoneからSomeへの更新のみ許可
            if existing.is_none() {
                map.insert(pointer.to_string(), content.map(|c| c.to_string()));
            }
            false
        } else {
            // 新規登録
            map.insert(pointer.to_string(), content.map(|c| c.to_string()));
            true
        }
    }

    /// コンテンツが設定済みのスキーマを全て取り出す
    /// 取り出したエントリのvalueはNoneに戻る（再度drainされることを防ぐ）
    pub fn drain(&self) -> HashMap<String, Option<String>> {
        let mut map = self.0.lock().unwrap();
        let mut drained = HashMap::new();
        for (key, value) in map.iter_mut() {
            if let Some(v) = value.take() {
                drained.insert(key.clone(), Some(v));
            }
        }
        drained
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_drain() {
        let cache = SchemaCache::new();
        // 1回目: Noneで登録（新規）
        assert!(cache.push("#/components/schemas/Pet", None));
        // 2回目: Someで更新（既存だがNone→Someは許可）
        assert!(!cache.push("#/components/schemas/Pet", Some("struct Pet{}")));
        // 3回目: 重複登録（既にSomeなので変更なし）
        assert!(!cache.push("#/components/schemas/Pet", Some("struct Pet2{}")));

        let drained = cache.drain();
        assert_eq!(drained.len(), 1);
        assert_eq!(
            drained.get("#/components/schemas/Pet"),
            Some(&Some("struct Pet{}".to_string()))
        );

        // drain後は再度drainしても空
        let drained2 = cache.drain();
        assert!(drained2.is_empty());
    }
}
