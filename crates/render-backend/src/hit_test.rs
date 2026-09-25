//! 全局命中测试器 (Hit Testing Pipeline)

use element_core::NodeId;
use layout_engine::LayoutRect;

#[derive(Clone, Debug)]
pub struct HitCandidate {
    pub node_id: NodeId,
    pub bounds: LayoutRect,
    pub z_index: i32,
    pub absorbs_clicks: bool,
}

pub struct HitTester;

impl HitTester {
    /// 根据物理坐标 (x, y) 倒序命中测试，找到最上层且响应点击的节点
    pub fn find_topmost_node(candidates: &[HitCandidate], x: f32, y: f32) -> Option<NodeId> {
        // 先按 z_index 降序排序，同一层按在数组中后声明优先
        let mut sorted: Vec<&HitCandidate> = candidates.iter().collect();
        sorted.sort_by(|a, b| b.z_index.cmp(&a.z_index));

        for item in sorted {
            if item.absorbs_clicks && item.bounds.contains(x, y) {
                return Some(item.node_id);
            }
        }
        None
    }
}
