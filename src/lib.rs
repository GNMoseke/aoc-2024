pub mod template;

pub mod file_util {
    // Use this file to add helper functions and additional modules.
    pub fn split_to_arrays(raw: &str) -> (Vec<u32>, Vec<u32>) {
        let mut first = Vec::new();
        let mut second = Vec::new();
        for line in raw.split('\n') {
            let mut split = line.trim().split_whitespace();
            first.push(split.next().unwrap().parse::<u32>().unwrap());
            second.push(split.next().unwrap().parse::<u32>().unwrap());
        }
        return (first, second);
    }
}
