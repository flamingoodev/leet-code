use std::collections::HashMap;

/// #1 两数之和
/// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那两个整数，并返回他们的数组下标。
///
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
///
/// 示例:
///
/// 给定 nums = [2, 7, 11, 15], target = 9
///
/// 因为 nums[0] + nums[1] = 2 + 7 = 9
/// 所以返回 [0, 1]

/// 方法一：暴力法
/// 暴力法很简单，遍历每个元素 x，并查找是否存在一个值与 target - x 相等的目标元素。
///
/// 复杂度分析：
///
/// 时间复杂度：O(n^2)。
///
/// 对于每个元素，我们试图通过遍历数组的其余部分来寻找它所对应的目标元素，这将耗费 O(n)的时间。
/// 因此时间复杂度为 O(n^2)。
///
/// 空间复杂度：O(1)。
///
#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let l = nums.len();
    let mut a = vec![0; l];
    for i in 0..l {
        for i1 in 0..i {
            if nums[i] == a[i1] {
                let result = vec![i1 as i32, i as i32];
                return result;
            }
        }
        a[i] = target - nums[i];
    }
    [0, 0].to_vec()
}

/// 方法二：两遍哈希表
///
/// 为了对运行时间复杂度进行优化，我们需要一种更有效的方法来检查数组中是否存在目标元素。如果存在，我们需要找出它的索引。
/// 保持数组中的每个元素与其索引相互对应的最好方法是什么？哈希表。
///
/// 通过以空间换取速度的方式，我们可以将查找时间从 O(n) 降低到 O(1)。哈希表正是为此目的而构建的，
/// 它支持以 近似 恒定的时间进行快速查找。我用“近似”来描述，是因为一旦出现冲突，查找用时可能会退化到 O(n)。
/// 但只要你仔细地挑选哈希函数，在哈希表中进行查找的用时应当被摊销为 O(1)。
///
/// 一个简单的实现使用了两次迭代。在第一次迭代中，我们将每个元素的值和它的索引添加到表中。
/// 然后，在第二次迭代中，我们将检查每个元素所对应的目标元素（target - nums[i]）是否存在于表中。
/// 注意，该目标元素不能是 nums[i] 本身！
///
#[allow(dead_code)]
fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for i in 0..nums.len() {
        if let Some(k) = map.get(&(target - nums[i])) {
            if *k != i {
                return vec![*k as i32, i as i32];
            }
        }
        map.insert(nums[i], i);
    }
    panic!("NOT FOUND!");
}

#[test]
fn two_sum_test() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum1(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
