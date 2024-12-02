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

    pub fn split_to_nested_vec(raw: &str) -> Vec<Vec<u32>> {
        let mut res = Vec::<Vec<u32>>::new();
        for line in raw.split('\n') {
            let mut vec = Vec::<u32>::new();
            line.trim()
                .split_whitespace()
                .for_each(|num| vec.push(num.parse::<u32>().unwrap()));
            res.push(vec);
        }
        return res;
    }
}
