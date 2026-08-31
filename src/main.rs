use puzzle_solver::*;
use std::env;

/// 将数字字符串按每三位加逗号格式化，便于阅读
///
/// # 参数
/// * `s` - 原始数字字符串
///
/// # 返回值
/// 返回格式化后的字符串，每三位用逗号分隔
fn format_number(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("用法: puzzle_solver <棋盘字符串>");
        println!("示例: puzzle_solver 504871623");
        return;
    }

    let puzzle_string = &args[1];

    let board = match parse_input(puzzle_string) {
        Ok(b) => b,
        Err(e) => {
            println!("错误：{}", e);
            println!("请确保输入格式正确（例如：123456780）");
            return;
        }
    };

    if !is_solvable(board) {
        println!("此棋盘无解！");
        return;
    }

    if let Some(solution) = solve_astar(board) {
        if solution.is_empty() {
            println!("已经是目标状态！");
        } else {
            println!("解法（共 {} 步）：", solution.len());
            let solution_str = solution.iter().map(|x| x.to_string()).collect::<String>();
            println!("{}", format_number(&solution_str));
        }
    } else {
        println!("无解！");
    }
}

#[cfg(test)]
mod tests {
    use puzzle_solver::*;
    use super::format_number;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number("41326873268754123541236"), "413,268,732,687,541,235,412,36");
        assert_eq!(format_number("123"), "123");
        assert_eq!(format_number("1234"), "123,4");
        assert_eq!(format_number(""), "");
        assert_eq!(format_number("123456"), "123,456");
        assert_eq!(format_number("1234567"), "123,456,7");
        assert_eq!(format_number("12345678"), "123,456,78");
        assert_eq!(format_number("1234567890"), "123,456,789,0");
        assert_eq!(format_number("57578"), "575,78");
        assert_eq!(format_number("34682"), "346,82");
    }

    #[test]
    fn test_board_creation() {
        let board = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        let expected = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_board_equality() {
        let board1 = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        let board2 = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        let board3 = Board::from_array([1, 2, 3, 4, 5, 6, 7, 0, 8]);

        assert_eq!(board1, board2);
        assert_ne!(board1, board3);
    }

    #[test]
    fn test_manhattan_distance() {
        let solved = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        let simple = Board::from_array([1, 2, 3, 4, 5, 6, 7, 0, 8]);
        let hard = Board::from_array([8, 3, 2, 1, 4, 5, 6, 7, 0]);

        assert_eq!(solved.manhattan_distance(&Board::GOAL_POSITIONS), 0);
        assert_eq!(simple.manhattan_distance(&Board::GOAL_POSITIONS), 1);
        assert_eq!(hard.manhattan_distance(&Board::GOAL_POSITIONS), 12);
    }

    #[test]
    fn test_is_solvable() {
        let solvable1 = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        let solvable2 = Board::from_array([1, 2, 3, 4, 5, 6, 7, 0, 8]);
        let unsolvable = Board::from_array([1, 2, 3, 4, 5, 6, 8, 7, 0]);

        assert!(is_solvable(solvable1));
        assert!(is_solvable(solvable2));
        assert!(!is_solvable(unsolvable));
    }

    #[test]
    fn test_parse_input() {
        let valid = "123456780";
        let result = parse_input(valid);
        assert!(result.is_ok());
        let expected = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        assert_eq!(result.unwrap(), expected);

        let invalid_length = "12345678";
        assert!(parse_input(invalid_length).is_err());

        let invalid_chars = "12345678a";
        assert!(parse_input(invalid_chars).is_err());

        let invalid_duplicate = "112345678";
        assert!(parse_input(invalid_duplicate).is_err());

        let missing_zero = "123456789";
        assert!(parse_input(missing_zero).is_err());

        let out_of_range = "012345679";
        assert!(parse_input(out_of_range).is_err());
    }

    #[test]
    fn test_solve_astar_already_solved() {
        let solved = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        let result = solve_astar(solved);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn test_solve_astar_simple_case() {
        let simple = Board::from_array([1, 2, 3, 4, 5, 6, 7, 0, 8]);
        let result = solve_astar(simple);
        assert!(result.is_some());
        let path = result.unwrap();
        assert!(!path.is_empty());
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], 8);
    }

    #[test]
    fn test_solve_astar_two_steps() {
        let board = Board::from_array([1, 2, 3, 4, 5, 6, 0, 7, 8]);
        let result = solve_astar(board);
        assert!(result.is_some());
        let path = result.unwrap();
        assert!(!path.is_empty());
        assert_eq!(path.len(), 2);
        assert_eq!(path, vec![7, 8]);
    }

    #[test]
    fn test_astar_node_creation() {
        let board = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        let h_cost = board.manhattan_distance(&Board::GOAL_POSITIONS);

        let node1 = AStarNode::new(board, 0, h_cost, None, 0);
        let node2 = AStarNode::new(board, 0, h_cost, None, 0);
        let node3 = AStarNode::new(board, 1, h_cost, Some(0), 0);
        assert_eq!(node1, node2);
        assert_ne!(node1, node3);
    }

    #[test]
    fn test_astar_creation() {
        let start = Board::from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        let mut astar = AStar::new(start);
        assert!(astar.find_lowest_f_cost().is_some());
    }
}
