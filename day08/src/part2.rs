use itertools::Itertools;

use glam::*;

pub fn process(src: &str) -> u64 {
    let mut positions = vec![];

    for line in src.lines() {
        let mut vals = line.split(',');
        let (x, y, z) = (
            vals.next().unwrap().parse::<i32>().unwrap(),
            vals.next().unwrap().parse::<i32>().unwrap(),
            vals.next().unwrap().parse::<i32>().unwrap(),
        );
        positions.push(ivec3(x, y, z));
    }

    let mut clusters: Vec<Vec<IVec3>> = vec![];

    for (a, b, _) in positions
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.as_vec3().distance_squared(b.as_vec3())))
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
    {
        let matches = clusters
            .iter()
            .positions(|cluster| {
                let contains_a = cluster.contains(a);
                let contains_b = cluster.contains(b);

                contains_a || contains_b
            })
            .collect::<Vec<usize>>();

        match matches.as_slice() {
            [] => {
                clusters.push(vec![*a, *b]);

                if clusters.len() == 1 && clusters[0].len() == positions.len() {
                    return a.x as u64 * b.x as u64;
                }
            }
            [index] => {
                let cluster = &clusters[*index];

                match (cluster.contains(a), cluster.contains(b)) {
                    (true, true) => {}
                    (true, false) => {
                        clusters[*index].push(*b);

                        if clusters.len() == 1 && clusters[0].len() == positions.len() {
                            return a.x as u64 * b.x as u64;
                        }
                    }
                    (false, true) => {
                        clusters[*index].push(*a);
                        if clusters.len() == 1 && clusters[0].len() == positions.len() {
                            return a.x as u64 * b.x as u64;
                        }
                    }
                    (false, false) => {}
                }
            }
            [index_a, index_b] => {
                let new_a = clusters.remove(*index_a.max(index_b));
                let new_b = clusters.remove(*index_a.min(index_b));
                let new: Vec<IVec3> = new_a
                    .into_iter()
                    .chain(new_b.into_iter())
                    .unique()
                    .collect();

                if new.len() == positions.len() {
                    return a.x as u64 * b.x as u64;
                }

                clusters.push(new);
            }
            _ => unreachable!(),
        }
    }

    0
}

#[test]
fn test() {
    let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    assert_eq!(process(input), 25272);
}
