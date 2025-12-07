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

    let mut beams = vec![v2(start.x, start.y + 1)];

    let mut splits = 0;

    loop {
        //print_grid(&grid);

        let mut new_beams = vec![];

        for beam in &beams {
            let mut split = false;
            match grid[beam.y + 1][beam.x] {
                Cell::Splitter => {
                    let new_beam_1 = v2(beam.x - 1, beam.y + 1);
                    let new_beam_2 = v2(beam.x + 1, beam.y + 1);

                    if grid[new_beam_1.y][new_beam_1.x] == Cell::Empty {
                        grid[new_beam_1.y][new_beam_1.x] = Cell::Beam;
                        new_beams.push(new_beam_1);
                        split = true;
                    }

                    if grid[new_beam_2.y][new_beam_2.x] == Cell::Empty {
                        grid[new_beam_2.y][new_beam_2.x] = Cell::Beam;
                        new_beams.push(new_beam_2);
                        split = true;
                    }
                }
                Cell::Empty => {
                    grid[beam.y + 1][beam.x] = Cell::Beam;
                    new_beams.push(v2(beam.x, beam.y + 1));
                }
                _ => {}
            }

            if split {
                splits += 1;
            }
        }

        //println!("{:?} {:?}", &beams, &new_beams);

        std::mem::swap(&mut beams, &mut new_beams);

        if beams[0].y + 1 >= grid.len() {
            break;
        }
    }

    splits
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for cell in row {
            match cell {
                Cell::Empty => print!("."),
                Cell::Start => print!("S"),
                Cell::Splitter => print!("^"),
                Cell::Beam => print!("|"),
            }
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

    assert_eq!(process(input), 21);
}
