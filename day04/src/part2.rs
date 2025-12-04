use glam::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Roll,
}

fn grid(src: &str) -> Vec<Vec<Cell>> {
    src.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll,
                    _ => panic!(),
                })
                .collect()
        })
        .collect::<Vec<_>>()
}

const SIDES: [IVec2; 8] = [
    ivec2(-1, -1),
    ivec2(0, -1),
    ivec2(1, -1),
    ivec2(-1, 0),
    ivec2(1, 0),
    ivec2(-1, 1),
    ivec2(0, 1),
    ivec2(1, 1),
];

fn valid(x: &IVec2, w: usize, h: usize) -> bool {
    x.x >= 0 && x.x < w as i32 && x.y >= 0 && x.y < h as i32
}

fn neighbours(grid: &Vec<Vec<Cell>>, x: IVec2) -> usize {
    let w = grid[0].len();
    let h = grid.len();

    let mut result = 0;

    for side in SIDES {
        let idx = x + side;

        if valid(&idx, w, h) {
            let val = grid[idx.y as usize][idx.x as usize];
            if val == Cell::Roll {
                result += 1;
            }
        }
    }

    result
}

pub fn process(src: &str) -> usize {
    let mut grid = grid(src);

    let mut cpt = 0;

    loop {
        let mut entries = vec![];

        for j in 0..grid.len() {
            for i in 0..grid[j].len() {
                if grid[j][i] == Cell::Roll {
                    let coords = ivec2(i as i32, j as i32);
                    if neighbours(&grid, coords) < 4 {
                        entries.push(coords);
                        cpt += 1;
                    }
                }
            }
        }

        if entries.is_empty() {
            break;
        } else {
            for entry in entries {
                grid[entry.y as usize][entry.x as usize] = Cell::Empty;
            }
        }
    }

    cpt
}

#[test]
fn test() {
    let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    assert_eq!(process(input), 43);
}

#[test]
fn parse_grid() {
    let input = r#".@.
@@.
..@"#;

    assert_eq!(
        grid(input),
        vec![
            vec![Cell::Empty, Cell::Roll, Cell::Empty],
            vec![Cell::Roll, Cell::Roll, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Roll],
        ]
    );
}

#[test]
fn test_neighbours() {
    let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    let grid = grid(input);

    assert_eq!(neighbours(&grid, ivec2(0, 0)), 2 as usize);
    assert_eq!(neighbours(&grid, ivec2(3, 3)), 7 as usize);
    assert_eq!(neighbours(&grid, ivec2(9, 8)), 4 as usize);
}
