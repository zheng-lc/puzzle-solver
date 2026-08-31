use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

// ──────────────────────────────────────────────
// Board
// ──────────────────────────────────────────────

/// 表示3x3八数码拼图的棋盘状态
///
/// 使用[u8; 9]数组存储棋盘，其中0表示空位，1-8表示拼图块
/// 线性数组存储，索引0-8对应3x3网格的左上到右下位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    data: [u8; 9],
}

impl Board {
    /// 从数组创建棋盘实例
    ///
    /// # 示例
    /// ```
    /// use puzzle_solver::Board;
    /// let board = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
    /// ```
    pub fn from_array(data: [u8; 9]) -> Self {
        Self { data }
    }

    /// 计算到目标棋盘的曼哈顿距离（使用预计算的目标位置表）
    ///
    /// 曼哈顿距离是每个拼图块当前位置到目标位置的水平距离
    /// 和垂直距离的总和，常用于启发式搜索算法
    ///
    /// # 参数
    /// * `goal_positions` - 预计算的目标位置映射表，goal_positions[value] = 该值在目标状态的位置
    ///
    /// # 返回值
    /// 返回曼哈顿距离值
    pub fn manhattan_distance(&self, goal_positions: &[usize; 9]) -> u32 {
        let mut distance = 0u32;
        for i in 0..9 {
            let value = self.data[i];
            if value != 0 {
                let current_row = i / 3;
                let current_col = i % 3;
                let goal_idx = goal_positions[value as usize];
                let goal_row = goal_idx / 3;
                let goal_col = goal_idx % 3;
                distance +=
                    ((current_row as i32 - goal_row as i32).abs()
                        + (current_col as i32 - goal_col as i32).abs()) as u32;
            }
        }
        distance
    }

    /// 预计算的目标位置表（公开常量，供外部测试使用）
    /// goal = [1, 2, 3, 4, 5, 6, 7, 8, 0]
    /// 值0→idx8, 值1→idx0, 值2→idx1, ..., 值8→idx7
    pub const GOAL_POSITIONS: [usize; 9] = [8, 0, 1, 2, 3, 4, 5, 6, 7];
}

// ──────────────────────────────────────────────
// AStarNode — A* 搜索节点
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AStarNode {
    board: Board,
    g_cost: u32,
    h_cost: u32,
    f_cost: u32,
    parent_index: Option<u32>,
    move_number: u8,
}

impl AStarNode {
    pub fn new(
        board: Board,
        g_cost: u32,
        h_cost: u32,
        parent_index: Option<u32>,
        move_number: u8,
    ) -> Self {
        Self {
            board,
            g_cost,
            h_cost,
            f_cost: g_cost + h_cost,
            parent_index,
            move_number,
        }
    }
}

// BinaryHeap 需要最大堆，所以我们反转比较
impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_cost
            .cmp(&self.f_cost)
            .then_with(|| other.h_cost.cmp(&self.h_cost))
    }
}

// ──────────────────────────────────────────────
// AStar — A* 搜索求解器
// ──────────────────────────────────────────────

/// A* 搜索算法求解器
///
/// 改进点：
/// - 使用 `BinaryHeap` 替代 Vec 线性扫描，find_min 从 O(n) 降至 O(log n)
/// - 使用 `g_scores` HashMap 避免 open_list 中的重复劣化路径
/// - 使用 `node_store: Vec<AStarNode>` 实现 O(1) 路径重构
/// - 使用预计算的 `goal_positions` 将曼哈顿距离从 O(9×n) 降至 O(n)
pub struct AStar {
    open_list: BinaryHeap<AStarNode>,
    closed_list: HashMap<Board, AStarNode>,
    node_store: Vec<AStarNode>,
    g_scores: HashMap<Board, u32>,
    goal_positions: [usize; 9],
}

impl AStar {
    /// 预计算目标状态 [1,2,3,4,5,6,7,8,0] 中每个值的目标位置
    /// goal_positions[v] = v 在目标状态中的索引
    fn default_goal_positions() -> [usize; 9] {
        // goal = [1, 2, 3, 4, 5, 6, 7, 8, 0]
        // 值0→idx8, 值1→idx0, 值2→idx1, ..., 值8→idx7
        [8, 0, 1, 2, 3, 4, 5, 6, 7]
    }

    pub fn new(start: Board) -> Self {
        let mut open_list = BinaryHeap::new();
        let closed_list = HashMap::new();
        let mut node_store = Vec::new();
        let mut g_scores = HashMap::new();
        let goal_positions = Self::default_goal_positions();

        let h_cost = start.manhattan_distance(&goal_positions);

        let start_node = AStarNode::new(start, 0, h_cost, None, 0);
        open_list.push(start_node);
        node_store.push(start_node);
        g_scores.insert(start, 0);

        Self {
            open_list,
            closed_list,
            node_store,
            g_scores,
            goal_positions,
        }
    }

    pub fn add_node(&mut self, board: Board, g_cost: u32, parent_board: Board, move_number: u8) {
        let h_cost = board.manhattan_distance(&self.goal_positions);

        let actual_parent_idx =
            self.node_store
                .iter()
                .position(|n| n.board == parent_board)
                .unwrap() as u32;

        let new_node =
            AStarNode::new(board, g_cost, h_cost, Some(actual_parent_idx), move_number);

        self.node_store.push(new_node);
        self.g_scores.insert(board, g_cost);
        self.open_list.push(new_node);
    }

    /// 弹出代价最低的节点（BinaryHeap O(log n)）
    pub fn find_lowest_f_cost(&mut self) -> Option<AStarNode> {
        self.open_list.pop()
    }

    /// 从目标状态重构路径（O(d) 复杂度）
    pub fn reconstruct_path(&self, goal: Board) -> Vec<u8> {
        let mut path = Vec::new();
        let mut current = goal;

        while let Some(node) = self.closed_list.get(&current) {
            if node.move_number != 0 {
                path.push(node.move_number);
            }

            if let Some(parent_index) = node.parent_index {
                current = self.node_store[parent_index as usize].board;
            } else {
                break;
            }
        }

        path.reverse();
        path
    }
}

// ──────────────────────────────────────────────
// Utility functions
// ──────────────────────────────────────────────

/// 检查棋盘是否可解
///
/// 使用8数码拼图的数学性质：如果逆序数为偶数，则棋盘可解
pub fn is_solvable(board: Board) -> bool {
    let mut inversions = 0u32;
    let values: Vec<u8> = board.data.iter().filter(|&&x| x != 0).copied().collect();

    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] > values[j] {
                inversions += 1;
            }
        }
    }

    inversions.is_multiple_of(2)
}

/// 解析用户输入为棋盘状态
pub fn parse_input(input: &str) -> Result<Board, String> {
    let trimmed = input.trim();

    if trimmed.len() != 9 {
        return Err(format!(
            "输入长度必须为9位字符，当前长度为 {} 位",
            trimmed.len()
        ));
    }

    let chars: Vec<char> = trimmed.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if !c.is_ascii_digit() {
            return Err(format!(
                "第 {} 位 '{}' 不是有效数字（0-9）",
                i + 1,
                c
            ));
        }
    }

    let digits: Vec<u8> = chars
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as u8)
        .collect();

    for (i, &d) in digits.iter().enumerate() {
        if d > 8 {
            return Err(format!(
                "第 {} 位 '{}' 超出有效范围（0-8），8 数码拼图只使用数字 0-8",
                i + 1,
                d
            ));
        }
    }

    let mut seen = [false; 10];
    for &d in &digits {
        if seen[d as usize] {
            return Err(format!(
                "数字 '{}' 重复出现，每个数字（0-9）只能使用一次",
                d
            ));
        }
        seen[d as usize] = true;
    }

    if !seen[0] {
        return Err("输入必须包含数字 0（代表空位）".to_string());
    }

    Ok(Board::from_array(digits.try_into().unwrap()))
}

/// 获取空格位置的有效移动位置
fn get_valid_moves(empty_pos: usize) -> Vec<usize> {
    let mut moves = Vec::new();
    let row = empty_pos / 3;
    let col = empty_pos % 3;

    if row > 0 {
        moves.push(empty_pos - 3);
    }
    if row < 2 {
        moves.push(empty_pos + 3);
    }
    if col > 0 {
        moves.push(empty_pos - 1);
    }
    if col < 2 {
        moves.push(empty_pos + 1);
    }

    moves
}

/// 使用A*算法求解3x3拼图
pub fn solve_astar(start: Board) -> Option<Vec<u8>> {
    let goal: Board = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);

    if start == goal {
        return Some(vec![]);
    }

    let mut astar = AStar::new(start);

    while let Some(current_node) = astar.find_lowest_f_cost() {
        // Lazy deletion: skip if this is a stale entry with worse g_cost
        if astar
            .g_scores
            .get(&current_node.board)
            .copied()
            .is_some_and(|g| g < current_node.g_cost)
        {
            continue;
        }

        if current_node.board == goal {
            astar.closed_list.insert(current_node.board, current_node);
            return Some(astar.reconstruct_path(goal));
        }

        astar.closed_list.insert(current_node.board, current_node);

        let empty_pos = current_node.board.data.iter().position(|&x| x == 0).unwrap();

        for num_pos in get_valid_moves(empty_pos) {
            let mut next_data = current_node.board.data;
            next_data.swap(empty_pos, num_pos);
            let next = Board::from_array(next_data);

            if astar.closed_list.contains_key(&next) {
                continue;
            }

            let g_cost = current_node.g_cost + 1;

            // Skip if we already have a better or equal path to this state
            if astar
                .g_scores
                .get(&next)
                .copied()
                .is_some_and(|g| g_cost >= g)
            {
                continue;
            }

            let move_number = next.data[empty_pos];

            astar.add_node(next, g_cost, current_node.board, move_number);
        }
    }

    None
}