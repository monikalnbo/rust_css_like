//! 客户端轻量持久化与会话存储 (LocalStorage & SessionStore)
//!
//! 对标 HTML localStorage 与 PHP $_SESSION，提供内存缓存与单文件持久化存盘能力。

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq)]
pub struct StorageEntry {
    pub value: String,
    pub expires_at: Option<u64>,
}

impl StorageEntry {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            expires_at: None,
        }
    }

    pub fn with_ttl(value: impl Into<String>, ttl_secs: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            value: value.into(),
            expires_at: Some(now + ttl_secs),
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            return now >= exp;
        }
        false
    }
}

pub struct LocalStorage {
    file_path: Option<PathBuf>,
    entries: HashMap<String, StorageEntry>,
}

impl LocalStorage {
    pub fn new_in_memory() -> Self {
        Self {
            file_path: None,
            entries: HashMap::new(),
        }
    }

    pub fn new_persistent(path: impl AsRef<Path>) -> Self {
        let mut storage = Self {
            file_path: Some(path.as_ref().to_path_buf()),
            entries: HashMap::new(),
        };
        storage.load_from_disk();
        storage
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(key.into(), StorageEntry::new(value));
        self.save_to_disk();
    }

    pub fn set_ex(&mut self, key: impl Into<String>, value: impl Into<String>, ttl_secs: u64) {
        self.entries
            .insert(key.into(), StorageEntry::with_ttl(value, ttl_secs));
        self.save_to_disk();
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).and_then(|e| {
            if e.is_expired() {
                None
            } else {
                Some(e.value.as_str())
            }
        })
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        let removed = self.entries.remove(key).map(|e| e.value);
        if removed.is_some() {
            self.save_to_disk();
        }
        removed
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.save_to_disk();
    }

    pub fn keys(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter(|(_, e)| !e.is_expired())
            .map(|(k, _)| k.clone())
            .collect()
    }

    fn save_to_disk(&self) {
        if let Some(path) = &self.file_path {
            let mut serialized = String::new();
            for (k, v) in &self.entries {
                if !v.is_expired() {
                    let exp = v.expires_at.unwrap_or(0);
                    serialized.push_str(&format!("{k}\t{exp}\t{}\n", v.value));
                }
            }
            let _ = fs::write(path, serialized);
        }
    }

    fn load_from_disk(&mut self) {
        if let Some(path) = &self.file_path {
            if let Ok(content) = fs::read_to_string(path) {
                for line in content.lines() {
                    let parts: Vec<&str> = line.splitn(3, '\t').collect();
                    if parts.len() == 3 {
                        let key = parts[0].to_string();
                        let exp: u64 = parts[1].parse().unwrap_or(0);
                        let value = parts[2].to_string();
                        let entry = StorageEntry {
                            value,
                            expires_at: if exp > 0 { Some(exp) } else { None },
                        };
                        if !entry.is_expired() {
                            self.entries.insert(key, entry);
                        }
                    }
                }
            }
        }
    }
}

pub struct SessionStore {
    session_id: String,
    data: HashMap<String, String>,
}

impl SessionStore {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            data: HashMap::new(),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn destroy(&mut self) {
        self.data.clear();
    }
}
