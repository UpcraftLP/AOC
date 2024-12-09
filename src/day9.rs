use crate::util::read_inputs;
use anyhow::Result;
use num_integer::Integer;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Segment {
    length: usize,
    id: Option<u16>,
}

fn parse_inputs(inputs: &[String]) -> Vec<Segment> {
    inputs
        .first()
        .expect("No input provided")
        .char_indices()
        .map(|(idx, c)| {
            let length = c.to_digit(10).expect("Invalid digit") as usize;
            let id = if idx.is_even() {
                Some((idx / 2) as u16)
            } else {
                None
            };
            Segment { length, id }
        })
        .collect()
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(9)?;

    let mut result: i64;

    let mut segments = parse_inputs(&inputs);

    let mut i = 0;
    while i < segments.len() - 1 {
        let segment = segments[i];
        if segment.id.is_none() {
            let mut remaining_space = segment.length;
            while remaining_space > 0 {
                let mut segment2 = segments.pop().unwrap();
                // drop trailing empty segments
                while segment2.id.is_none() {
                    segment2 = segments.pop().unwrap();
                }

                // move data to new spot
                segments[i].id = segment2.id;

                // segment2 fits fully in segment1
                if remaining_space >= segment2.length {
                    remaining_space -= segment2.length;

                    if remaining_space > 0 {
                        // shrink segment1
                        segments[i].length = segment2.length;

                        // insert new segment to fill remaining empty space
                        segments.insert(
                            i + 1,
                            Segment {
                                length: remaining_space,
                                id: None,
                            },
                        );
                        i += 1;
                    }
                } else {
                    // put remaining data back at the end
                    segments.push(Segment {
                        length: segment2.length - remaining_space,
                        id: segment2.id,
                    });
                    break;
                }
            }
        }

        i += 1;
    }

    result = checksum(&segments);

    println!("Day 9 Part 1: {result}");

    // ---------------------------------------

    let mut segments = parse_inputs(&inputs);

    let mut i = segments.len() - 1;
    while i > 0 {
        let segment = segments[i];

        if segment.id.is_some() {
            for j in 0..segments.len() {
                // no suitable place found
                if j >= i {
                    break;
                }

                let segment2 = segments[j];
                if segment2.id.is_none() && segment2.length >= segment.length {
                    segments.swap(i, j);

                    if segment2.length != segment.length {
                        // 2 is smaller than 1
                        let diff = segment2.length - segment.length;

                        // update size of old object (empty space, now in old pos)
                        segments[i].length = segment.length;

                        // insert new object to fill remaining empty space in new pos
                        segments.insert(
                            j + 1,
                            Segment {
                                length: diff,
                                id: None,
                            },
                        );
                        i += 1;
                    }

                    break;
                }
            }
        }

        i -= 1;
    }

    result = checksum(&segments);

    println!("Day 9 Part 2: {result}");

    Ok(())
}

fn checksum(memory: &[Segment]) -> i64 {
    let mut result = 0;
    let mut pos = 0;
    for seg in memory.iter() {
        if let Some(id) = seg.id {
            for i in 0..seg.length {
                result += (pos + i) as i64 * id as i64;
            }
        }

        pos += seg.length;
    }
    result
}
