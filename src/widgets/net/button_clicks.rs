#[derive(Default, Clone)]
pub struct ButtonClicks {
    pub reset: bool,
    pub create: bool,
    pub color_cones: bool,
    pub color_cycles: bool,
    pub export_dot: bool,
    pub export_svg: bool,
    pub delete_cone: bool,
    pub delete_cycles: bool,
    pub color_nodes_and_edges: bool,
    pub delete_nodes_and_edges: bool,
    pub history_go_up: bool,
    pub history_go_down: bool,
    pub history_go_sibling: bool,
}