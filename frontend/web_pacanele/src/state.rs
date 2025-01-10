use rules::Fruit;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PcnlState {
    pub wheels: Vec<PcnlWheelState>,
    pub money: u64,
    pub last_win: Option<u16>,
    pub last_messages: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PcnlWheelState {
    pub pcnl_id: u32,
    pub pcnl_count: u32,
    pub new_fruit: rules::Fruit,
    pub old_fruit: rules::Fruit,
    pub spin_count: u32,
    pub new_idx: u32,
    pub old_idx: u32,
    pub spin_period: f64,
    pub wheel_stage: WheelStage,
    pub rotations_diff: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub shuffle: Vec<Fruit>,
    pub idx: HashMap<Fruit, u32>,
}
