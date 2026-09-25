//! 表单控件族内部状态机与双向绑定中枢 (Form Controls & Two-Way Binding)

use crate::node::NodeId;
use std::collections::HashMap;

/// 复选框状态机
#[derive(Clone, Debug, PartialEq)]
pub struct CheckboxState {
    pub checked: bool,
    pub bind_var: Option<String>,
}

impl CheckboxState {
    pub fn new(checked: bool, bind_var: Option<String>) -> Self {
        Self { checked, bind_var }
    }

    pub fn toggle(&mut self) -> bool {
        self.checked = !self.checked;
        self.checked
    }
}

/// 单选框状态机
#[derive(Clone, Debug, PartialEq)]
pub struct RadioState {
    pub group_name: String,
    pub value: String,
    pub selected: bool,
    pub bind_var: Option<String>,
}

impl RadioState {
    pub fn new(group_name: impl Into<String>, value: impl Into<String>, selected: bool) -> Self {
        Self {
            group_name: group_name.into(),
            value: value.into(),
            selected,
            bind_var: None,
        }
    }
}

/// 下拉选择菜单状态机
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectState {
    pub is_open: bool,
    pub options: Vec<(String, String)>, // (value, label)
    pub selected_index: Option<usize>,
    pub bind_var: Option<String>,
}

impl SelectState {
    pub fn new(options: Vec<(String, String)>) -> Self {
        Self {
            is_open: false,
            options,
            selected_index: None,
            bind_var: None,
        }
    }

    pub fn toggle_open(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn select(&mut self, index: usize) -> Option<&str> {
        if index < self.options.len() {
            self.selected_index = Some(index);
            self.is_open = false;
            Some(&self.options[index].0)
        } else {
            None
        }
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_index
            .and_then(|idx| self.options.get(idx).map(|(v, _)| v.as_str()))
    }
}

/// 滑动条状态机
#[derive(Clone, Debug, PartialEq)]
pub struct SliderState {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub value: f32,
    pub bind_var: Option<String>,
}

impl SliderState {
    pub fn new(min: f32, max: f32, step: f32, initial_val: f32) -> Self {
        Self {
            min,
            max,
            step,
            value: initial_val.clamp(min, max),
            bind_var: None,
        }
    }

    /// 根据拖拽比例 (0.0 ~ 1.0) 更新数值并按 step 对齐
    pub fn set_by_ratio(&mut self, ratio: f32) -> f32 {
        let r = ratio.clamp(0.0, 1.0);
        let raw = self.min + (self.max - self.min) * r;
        let stepped = if self.step > 0.0 {
            ((raw - self.min) / self.step).round() * self.step + self.min
        } else {
            raw
        };
        self.value = stepped.clamp(self.min, self.max);
        self.value
    }
}

/// 控件特定状态枚举
#[derive(Clone, Debug, PartialEq)]
pub enum FormControlKind {
    Checkbox(CheckboxState),
    Radio(RadioState),
    Select(SelectState),
    Slider(SliderState),
}

/// 全局表单控件状态中枢
#[derive(Clone, Debug, Default)]
pub struct FormRegistry {
    controls: HashMap<NodeId, FormControlKind>,
}

impl FormRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, node_id: NodeId, control: FormControlKind) {
        self.controls.insert(node_id, control);
    }

    pub fn get(&self, node_id: NodeId) -> Option<&FormControlKind> {
        self.controls.get(&node_id)
    }

    pub fn get_mut(&mut self, node_id: NodeId) -> Option<&mut FormControlKind> {
        self.controls.get_mut(&node_id)
    }

    /// 触发复选框点击切换
    pub fn toggle_checkbox(&mut self, node_id: NodeId) -> Option<(bool, Option<String>)> {
        if let Some(FormControlKind::Checkbox(cb)) = self.controls.get_mut(&node_id) {
            let new_state = cb.toggle();
            let bind = cb.bind_var.clone();
            Some((new_state, bind))
        } else {
            None
        }
    }

    /// 触发单选框选中，自动排他同组其他单选框
    pub fn select_radio(&mut self, node_id: NodeId) -> Option<String> {
        let group_name = if let Some(FormControlKind::Radio(r)) = self.controls.get(&node_id) {
            r.group_name.clone()
        } else {
            return None;
        };

        // 排他同组
        for (_, ctrl) in self.controls.iter_mut() {
            if let FormControlKind::Radio(r) = ctrl {
                if r.group_name == group_name {
                    r.selected = false;
                }
            }
        }

        if let Some(FormControlKind::Radio(r)) = self.controls.get_mut(&node_id) {
            r.selected = true;
            Some(r.value.clone())
        } else {
            None
        }
    }
}
