use std::collections::HashMap;

pub fn run() {
    // 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、
    // 中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希函数会很有帮助）。

    let v = vec![1, 6, 7, 8, 34, 200, 34, 6, 9];
    // 1. 平均数 1 6 6 7 8 9 34 34 200
    // 2. 中位数 8
    // 3. 众数   6

    fn average(nums: &Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let mut sum = 0;
        for n in nums {
            sum += n;
        }
        sum / len
    }

    let average_num = average(&v);

    println!("平均数:{}", average_num);

    fn center_number(nums: &Vec<i32>) -> i32 {
        let mut _nums = nums.clone();
        let len = nums.len();
        let divisor = len / 2;
        let mod_number = len % 2;
        let index = divisor + mod_number;
        _nums[index - 1]
    }

    println!("中位数:{}", center_number(&v));

    fn mode_number(nums: &Vec<i32>) -> i32 {
        let mut count_map: HashMap<i32, i8> = HashMap::new();
        for n in nums {
            let counter = count_map.entry(n.clone()).or_insert(0);
            *counter += 1;
        }
        let mut cur_count: i8 = 0;
        let mut res: i32 = 0;
        for (k, v) in count_map {
            if v > cur_count {
                cur_count = v;
                res = k;
            }
        }
        res
    }

    println!("众数:{}", mode_number(&v));

    // 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
    // 所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。
    // 牢记 UTF-8 编码！

    // 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
    // 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，
    // 或者公司每个部门的所有员工按照字典序排列的列表。
}
