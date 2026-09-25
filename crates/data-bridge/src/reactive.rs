//! UI 与数据源响应式数据绑定 (Reactive State & Data Binding)

use std::sync::{Arc, Mutex};

/// 响应式信号单元 (Signal)
#[derive(Clone, Debug)]
pub struct Signal<T> {
    value: Arc<Mutex<T>>,
}

impl<T: Clone> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn get(&self) -> T {
        self.value.lock().unwrap().clone()
    }

    pub fn set(&self, new_val: T) {
        *self.value.lock().unwrap() = new_val;
        // 在此处可触发 UI 脏标记刷新
    }
}
