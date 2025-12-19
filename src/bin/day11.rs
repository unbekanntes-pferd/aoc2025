use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../assets/day11/input.txt");
    let part1 = solve_part1(input);

    println!("Part 1: {part1}");
    
    let part2 = solve_part2(input);

    println!("Part 2: {part2}");
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum OutConn {
    Start,
    Device(String),
    End,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Device {
    name: String,
    out: Vec<OutConn>,
}

impl Device {
    fn new(name: impl Into<String>, out: Vec<OutConn>) -> Self {
        Self {
            name: name.into(),
            out,
        }
    }

    fn is_start(&self) -> bool {
        return self.name == "you";
    }

    fn goes_out(&self) -> bool {
        self.out.iter().any(|conn| conn == &OutConn::End)
    }
}

fn parse_device(line: &str) -> Option<Device> {
    let name = line.split(':').next()?;
    let out = line
        .split(':')
        .skip(1)
        .next()?
        .split_ascii_whitespace()
        .map(|name| match name {
            "you" => OutConn::Start,
            "out" => OutConn::End,
            name => OutConn::Device(name.to_string()),
        })
        .collect();

    Some(Device::new(name, out))
}

fn parse_input(input: &str) -> HashMap<String, Device> {
    input
        .lines()
        .flat_map(parse_device)
        .map(|device| (device.name.clone(), device))
        .collect()
}

fn solve_part1(input: &str) -> usize {
    let devices = parse_input(input);

    if let Some(start) = devices.get("you") {
        return find_paths(start, &devices);
    }

    0
}

fn solve_part2(input: &str) -> usize {
    let devices = parse_input(input);
    if let Some(start) = devices.get("svr") {
        dbg!(&start);
        return find_paths_crossing_dac_fft(start, &devices);
    }

    0
}

fn find_paths_crossing_dac_fft(current: &Device, devices: &HashMap<String, Device>) -> usize {
    let mut visited = HashSet::new();
    let mut cache: HashMap<(String, bool, bool), usize> = HashMap::new();
    

    fn recursive_path_search(
        current: &Device,
        devices: &HashMap<String, Device>,
        visited: &mut HashSet<String>,
        seen: (bool, bool),
        cache: &mut HashMap<(String, bool, bool), usize>
    ) -> usize {
        let mut count = 0;
        visited.insert(current.name.clone());
        
        if let Some(count) = cache.get(&(current.name.clone(), seen.0, seen.1)) {
            return *count
        }

        let mut seen_dac = seen.0;
        let mut seen_fft = seen.1;
        for outgoing in current.out.iter() {
            let new_count = match outgoing {
                OutConn::Device(name) => {  
                    if visited.contains(name) {
                        dbg!("yes this is a cycle");
                        continue;
                    }
                    let Some(device) = devices.get(name) else {
                        continue;
                    };
                    
                    if name == "dac" {
                        seen_dac = true;
                    }
                    
                    if name == "fft" {
                        seen_fft = true;
                    }

                    recursive_path_search(&device, devices, visited, (seen_dac, seen_fft), cache)
                }
                OutConn::Start => {
                    if visited.contains("you") {
                        continue;
                    }
                    let Some(device) = devices.get("you") else {
                        continue;
                    };

                    recursive_path_search(&device, devices, visited, (seen_dac, seen_fft), cache)
                    
                },
                OutConn::End => {
                    if seen_dac && seen_fft {
                        1
                    } else {
                        0
                    }
                }
            };

            count += new_count;
        }

        visited.remove(&current.name);
        
        cache.insert((current.name.clone(), seen.0, seen.1), count);

        count
    }

    recursive_path_search(current, devices, &mut visited, (false, false), &mut cache)
}

fn find_paths(current: &Device, devices: &HashMap<String, Device>) -> usize {
    let mut visited = HashSet::new();

    fn recursive_path_search(
        current: &Device,
        devices: &HashMap<String, Device>,
        visited: &mut HashSet<String>,
    ) -> usize {
        let mut count = 0;
        visited.insert(current.name.clone());

        for outgoing in current.out.iter() {
            let new_count = match outgoing {
                OutConn::Device(name) => {
                    let Some(device) = devices.get(name) else {
                        continue;
                    };

                    if visited.contains(name) {
                        continue;
                    }

                    recursive_path_search(&device, devices, visited)
                }
                OutConn::Start => 0,
                OutConn::End => 1,
            };
            count += new_count;
        }

        visited.remove(&current.name);

        count
    }

    recursive_path_search(current, devices, &mut visited)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input = include_str!("../../assets/day11/test.txt");

        let devices = parse_input(input);

        assert_eq!(devices.len(), 10);

        assert_eq!(
            devices.get("aaa").unwrap(),
            &Device::new(
                "aaa",
                vec![OutConn::Start, OutConn::Device("hhh".to_string())]
            )
        );
        assert_eq!(
            devices.get("you").unwrap(),
            &Device::new(
                "you",
                vec![
                    OutConn::Device("bbb".to_string()),
                    OutConn::Device("ccc".to_string())
                ]
            )
        );
        assert_eq!(
            devices.get("bbb").unwrap(),
            &Device::new(
                "bbb",
                vec![
                    OutConn::Device("ddd".to_string()),
                    OutConn::Device("eee".to_string())
                ]
            )
        );
        assert_eq!(
            devices.get("ccc").unwrap(),
            &Device::new(
                "ccc",
                vec![
                    OutConn::Device("ddd".to_string()),
                    OutConn::Device("eee".to_string()),
                    OutConn::Device("fff".to_string())
                ]
            )
        );
        assert_eq!(
            devices.get("ddd").unwrap(),
            &Device::new("ddd", vec![OutConn::Device("ggg".to_string())])
        );
        assert_eq!(
            devices.get("eee").unwrap(),
            &Device::new("eee", vec![OutConn::End])
        );
        assert_eq!(
            devices.get("fff").unwrap(),
            &Device::new("fff", vec![OutConn::End])
        );
        assert_eq!(
            devices.get("ggg").unwrap(),
            &Device::new("ggg", vec![OutConn::End])
        );
        assert_eq!(
            devices.get("hhh").unwrap(),
            &Device::new(
                "hhh",
                vec![
                    OutConn::Device("ccc".to_string()),
                    OutConn::Device("fff".to_string()),
                    OutConn::Device("iii".to_string())
                ]
            )
        );
        assert_eq!(
            devices.get("iii").unwrap(),
            &Device::new("iii", vec![OutConn::End])
        );
        assert!(devices.get("you").unwrap().is_start());
        assert!(devices.get("eee").unwrap().goes_out());
        assert!(devices.get("fff").unwrap().goes_out());
        assert!(devices.get("ggg").unwrap().goes_out());
        assert!(devices.get("iii").unwrap().goes_out());
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../../assets/day11/test.txt");

        let result = solve_part1(input);

        assert_eq!(5, result);
    }
    
    #[test]
    fn test_solve_part2() {
        let input = include_str!("../../assets/day11/test2.txt");

        let result = solve_part2(input);

        assert_eq!(2, result);
    }
}
