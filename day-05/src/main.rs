use rayon::prelude::*;

#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Mapper {
    length: u64,
    source: u64,
    destination: u64,
}

impl Mapper {
    #[inline]
    pub const fn map_to_destination(&self, i: u64) -> u64 {
        if self.in_range(i) {
            let dist = i - self.source;
            self.destination + dist
        } else {
            i
        }
    }

    #[inline]
    pub const fn in_range(&self, i: u64) -> bool {
        self.source <= i && i < self.source + self.length
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Lookup(Vec<Mapper>);

impl Lookup {
    #[inline]
    pub fn map_to_destination(&self, i: u64) -> u64 {
        self.0
            .iter()
            .find(|mapper| mapper.in_range(i))
            .map_or(i, |mapper| mapper.map_to_destination(i))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Table {
    seed_to_soil: Lookup,
    soil_to_fertilizer: Lookup,
    fertilizer_to_water: Lookup,
    water_to_light: Lookup,
    light_to_temperature: Lookup,
    temperature_to_humidity: Lookup,
    humidity_to_location: Lookup,
}

impl Table {
    pub fn seed_to_location(&self, i: u64) -> u64 {
        let soil = self.seed_to_soil.map_to_destination(i);
        let fertilizer = self.soil_to_fertilizer.map_to_destination(soil);
        let water = self.fertilizer_to_water.map_to_destination(fertilizer);
        let light = self.water_to_light.map_to_destination(water);
        let temperature = self.light_to_temperature.map_to_destination(light);
        let humidity = self.temperature_to_humidity.map_to_destination(temperature);
        self.humidity_to_location.map_to_destination(humidity)
    }
}

#[derive(Debug, PartialEq, Default)]
struct InputData {
    seeds: Vec<u64>,
    table: Table,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        bytes::complete::tag,
        character::complete::{anychar, line_ending, space1, u64},
        combinator::map,
        multi::{many_till, separated_list1},
        sequence::{pair, preceded, separated_pair, tuple},
    };
    let seeds = preceded(tag("seeds: "), separated_list1(space1, u64));
    let mapper = map(
        tuple((u64, space1, u64, space1, u64)),
        |(destination, _, source, _, length)| Mapper {
            length,
            source,
            destination,
        },
    );
    let mappers = separated_list1(line_ending, mapper);
    let lookup_id = many_till(anychar, line_ending);
    let lookup = map(pair(lookup_id, mappers), |((id, _), mappers)| {
        (id.into_iter().collect::<String>(), Lookup(mappers))
    });
    let table = map(
        separated_list1(pair(line_ending, line_ending), lookup),
        |table| {
            let mut my_table = Table::default();
            for (id, lookup) in table {
                match id.as_ref() {
                    "seed-to-soil map:" => my_table.seed_to_soil = lookup,
                    "soil-to-fertilizer map:" => my_table.soil_to_fertilizer = lookup,
                    "fertilizer-to-water map:" => my_table.fertilizer_to_water = lookup,
                    "water-to-light map:" => my_table.water_to_light = lookup,
                    "light-to-temperature map:" => my_table.light_to_temperature = lookup,
                    "temperature-to-humidity map:" => my_table.temperature_to_humidity = lookup,
                    "humidity-to-location map:" => my_table.humidity_to_location = lookup,
                    _ => unreachable!("Invalid lookup table"),
                }
            }
            my_table
        },
    );
    let mut parser = map(
        separated_pair(seeds, pair(line_ending, line_ending), table),
        |(seeds, table)| InputData { seeds, table },
    );

    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(InputData { seeds, table }: &InputData) -> AocResult<u64> {
    Ok(seeds
        .iter()
        .map(|seed| table.seed_to_location(*seed))
        .min()
        .expect("Some data"))
}

#[allow(clippy::unnecessary_wraps)]
fn part2(InputData { seeds, table }: &InputData) -> AocResult<u64> {
    Ok(seeds
        .par_chunks(2)
        .map(|v| (v[0]..(v[0] + v[1])))
        .flatten()
        .map(|seed| table.seed_to_location(seed))
        .min()
        .expect("Some data"))
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                seeds: vec![79, 14, 55, 13],
                table: Table {
                    seed_to_soil: Lookup(vec![
                        Mapper {
                            length: 2,
                            source: 98,
                            destination: 50
                        },
                        Mapper {
                            length: 48,
                            source: 50,
                            destination: 52
                        }
                    ]),
                    soil_to_fertilizer: Lookup(vec![
                        Mapper {
                            length: 37,
                            source: 15,
                            destination: 0
                        },
                        Mapper {
                            length: 2,
                            source: 52,
                            destination: 37
                        },
                        Mapper {
                            length: 15,
                            source: 0,
                            destination: 39
                        }
                    ]),
                    fertilizer_to_water: Lookup(vec![
                        Mapper {
                            length: 8,
                            source: 53,
                            destination: 49
                        },
                        Mapper {
                            length: 42,
                            source: 11,
                            destination: 0
                        },
                        Mapper {
                            length: 7,
                            source: 0,
                            destination: 42
                        },
                        Mapper {
                            length: 4,
                            source: 7,
                            destination: 57
                        }
                    ]),
                    water_to_light: Lookup(vec![
                        Mapper {
                            length: 7,
                            source: 18,
                            destination: 88
                        },
                        Mapper {
                            length: 70,
                            source: 25,
                            destination: 18
                        }
                    ]),
                    light_to_temperature: Lookup(vec![
                        Mapper {
                            length: 23,
                            source: 77,
                            destination: 45
                        },
                        Mapper {
                            length: 19,
                            source: 45,
                            destination: 81
                        },
                        Mapper {
                            length: 13,
                            source: 64,
                            destination: 68
                        }
                    ]),
                    temperature_to_humidity: Lookup(vec![
                        Mapper {
                            length: 1,
                            source: 69,
                            destination: 0
                        },
                        Mapper {
                            length: 69,
                            source: 0,
                            destination: 1
                        }
                    ]),
                    humidity_to_location: Lookup(vec![
                        Mapper {
                            length: 37,
                            source: 56,
                            destination: 60
                        },
                        Mapper {
                            length: 4,
                            source: 93,
                            destination: 56
                        }
                    ])
                }
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 35);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 46);
    }
}
