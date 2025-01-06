use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PcnlState {
    pub wheels: Vec<PcnlWheelState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PcnlWheelState {
    pub pcnl_id: u32,
    pub pcnl_count: u32,
    pub new_fruit: String,
    pub old_fruit: String,
    pub spin_count: u32,
    pub new_idx: u32,
    pub old_idx: u32,
    pub spin_period: f64,
    pub wheel_stage: WheelStage,
    pub rotations_diff: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WheelStage {
    Ready,
    PendingResults,
    HaveResults,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShuffleState {
    pub wheels: Vec<WheelShuffleState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WheelShuffleState {
    pub pcnl_id: u32,
    pub shuffle: Vec<String>,
    pub idx: HashMap<String, u32>,
}
