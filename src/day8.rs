use crate::util::read_inputs;
use anyhow::Result;
use glam::I64Vec3;
use itertools::Itertools;

const NUM_CONNECTIONS: usize = 1000;
const TOP_N_NETWORKS: usize = 3;

type Vec3 = I64Vec3;
type Num = i64;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(8)?;

    let boxes: Vec<Vec3> = inputs
        .iter()
        .map(|line| {
            Vec3::from_array(
                line.split(',')
                    .filter_map(|x| x.parse::<Num>().ok())
                    .collect_array::<3>()
                    .expect("invalid pos"),
            )
        })
        .collect_vec();

    let mut networks = boxes.iter().cloned().map(|pos| vec![pos]).collect_vec();

    let mut connections = boxes
        .into_iter()
        .tuple_combinations::<(Vec3, Vec3)>()
        .sorted_by_key(|&(a, b)| a.distance_squared(b));

    connections
        .by_ref()
        .take(NUM_CONNECTIONS)
        .for_each(|(start, end)| {
            let end_net = networks
                .extract_if(.., |net| net.contains(&end))
                .next()
                .expect("end position not in any network");

            // already linked
            if end_net.contains(&start) {
                networks.push(end_net);
                return;
            }

            let start_net = networks
                .iter_mut()
                .find(|net| net.contains(&start))
                .expect("start position not in any network");

            start_net.extend(end_net);
        });

    let result: usize = networks
        .iter()
        .map(|net| net.len())
        .sorted()
        .rev()
        .take(TOP_N_NETWORKS)
        .product();

    println!("Day 8 Part 1: {result}");

    // ---------------------------------------

    let (pos1, pos2) = connections
        .by_ref()
        .take_while_inclusive(|(start, end)| {
            let end_net = networks
                .extract_if(.., |net| net.contains(end))
                .next()
                .expect("end position not in any network");

            // already linked
            if end_net.contains(start) {
                networks.push(end_net);
                return true;
            }

            let start_net = networks
                .iter_mut()
                .find(|net| net.contains(start))
                .expect("start position not in any network");

            start_net.extend(end_net);

            networks.len() > 1
        })
        .last()
        .expect("network already full");

    let result = pos1.x * pos2.x;

    println!("Day 8 Part 2: {result}");

    Ok(())
}
