use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Start,
    Splitter,
    Beam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct V2 {
    x: usize,
    y: usize,
}

fn v2(x: usize, y: usize) -> V2 {
    V2 { x, y }
}

pub fn process(src: &str) -> u64 {
    let mut grid: Vec<Vec<Cell>> = Vec::new();

    let mut start = v2(0, 0);

    for (j, line) in src.lines().enumerate() {
        let mut row = vec![];

        for (i, ch) in line.chars().enumerate() {
            match ch {
                '.' => row.push(Cell::Empty),
                '^' => row.push(Cell::Splitter),
                'S' => {
                    row.push(Cell::Start);
                    start = v2(i, j);
                }
                _ => panic!("{:?} as [{}, {}] is invalid", ch, i, j),
            }
        }

        grid.push(row);
    }

    grid[start.y as usize + 1][start.x as usize] = Cell::Beam;

    let grid = grid;

    let mut dp = vec![vec![0u64; grid[0].len()]; grid.len()];

    dp[start.y + 1][start.x] = 1;

    for y in start.y + 2..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == Cell::Empty {
                dp[y][x] += dp[y - 1][x];
            }

            if grid[y][x] == Cell::Splitter {
                dp[y][x - 1] += dp[y - 1][x];
                dp[y][x + 1] += dp[y - 1][x];
            }
        }

        //print_dp(&dp);
    }

    dp.last().unwrap().iter().sum::<u64>()
}

fn print_dp(dp: &Vec<Vec<u64>>) {
    for r in dp {
        for c in r {
            print!("{c} ");
        }
        println!("");
    }
    println!("");
}

#[test]
fn test() {
    let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    assert_eq!(process(input), 40);
}
