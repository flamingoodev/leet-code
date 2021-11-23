use std::collections::BTreeMap;

/// (非leet code题目)
///
/// 顺序分组
/// 假如有N个从0开始编号的玩家，你需要将所有玩家等量的分为M组。
/// 若无法分组则返回 `{}`
///
/// 示例1: 有10个玩家，3个一组进行分组
///
/// 输入: 10 3
/// 输出:
/// 0 012
/// 1 345
/// 2 678
/// 3 9
/// 示例 2:
#[allow(dead_code)]
pub fn sequence_group(members: usize, group: usize) -> String {
    let mut count: usize = 0;
    let mut group_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut range = 0..members;
    let mut temp_vec: Vec<usize> = Vec::new();
    loop {
        for _ in 0..group {
            let option = range.next();
            if option == None {
                break;
            }
            temp_vec.push(option.unwrap());
        }
        if range.is_empty() && temp_vec.is_empty() {
            break;
        }
        group_map.insert(count, temp_vec.clone());
        temp_vec.clear();
        count += 1;
    }
    if group_map.is_empty() {
        return "{}".to_string();
    }
    for x in &group_map {
        let mut members = String::new();
        let mut iter = x.1.iter();
        loop {
            let next = iter.next();
            if next == None {
                break;
            }
            members += next.unwrap().to_string().as_str();
        }
        // output
        // println!("{} {}", x.0, members);
    }
    // for test
    format!("{:?}", group_map.clone())
}

#[test]
fn test() {
    assert_eq!("{0: [0, 1, 2], 1: [3, 4]}", sequence_group(5, 3));
    assert_eq!("{0: [0], 1: [1]}", sequence_group(2, 1));
    assert_eq!("{}", sequence_group(0, 0));
    assert_eq!("{0: [0], 1: [1]}", sequence_group(2, 1));
}
