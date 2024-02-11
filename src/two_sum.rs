use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) ->Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (num, i) in nums.iter().zip(0..nums.len()) {
        let difference = target - num;
        match map.get(&difference) {
            Some(j) => {
                return vec![*j, i as i32];
            },
            None => map.insert(*num, i as i32)
        };
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}