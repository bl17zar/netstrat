use petgraph::Direction;
use Direction::Outgoing;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum EdgeWeight {
    /// Fixed weight
    Fixed,
    /// Random weigh in range 0..1
    Random,
}

#[derive(PartialEq, Clone, Debug)]
pub struct NetSettings {
    pub ini_cnt: usize,
    pub fin_cnt: usize,
    pub total_cnt: usize,
    pub diamond_filter: bool,
    pub no_twin_edges: bool,
    pub max_out_degree: usize,
    pub edge_weight_type: EdgeWeight,
    pub edge_weight: f64,
}

impl Default for NetSettings {
    fn default() -> Self {
        Self {
            ini_cnt: 5,
            fin_cnt: 5,
            total_cnt: 20,
            max_out_degree: 3,
            no_twin_edges: true,
            diamond_filter: true,
            edge_weight_type: EdgeWeight::Fixed,
            edge_weight: 1.0,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct ConeSettings {
    pub roots_weights: Vec<String>,
    pub dir: Direction,
    pub max_steps: i32,
}

impl Default for ConeSettings {
    fn default() -> Self {
        Self {
            roots_weights: Default::default(),
            dir: Outgoing,
            max_steps: -1,
        }
    }
}