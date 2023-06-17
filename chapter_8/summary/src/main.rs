fn main() {
    println!("Summary Question 1");
    let sum_nums = vec![1, 2, 6, 6, 8, 3, 4, 6, 5, 4, 2];
    let (mode, median) = summary_1(sum_nums);
    println!("{:?} {:?}", mode, median);

    println!("Summary Question 2");
    println!("{}", summary_2("test"));
    println!("{}", summary_2("apple"));

    println!("Summary Question 3");
    summary_3();
}

// Given a vector return the median and mode
fn summary_1(mut nums: Vec<i32>) -> (Option<i32>, Option<i32>) {
    nums.sort();

    // [number, count]
    let mut max_mode = [None, None];
    let mut current_num = 0;
    let mut count = 0;

    // This is probably more performant than counting in a hash map
    for n in &nums {
        if current_num != *n {
            current_num = *n;
            count = 0;
        }
        count += 1;

        if max_mode[1] < Some(count) {
            max_mode[0] = Some(*n);
            max_mode[1] = Some(count);
        }
    }

    let mut median: Option<i32> = None;
    if nums.len() > 0 {
        let n = nums.len() as i32 / 2;
        median = Some(n);
    }
    (max_mode[0], median)
}

// Given a string make it piglatin
fn summary_2(s: &str) -> String {
    const vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let mut vowel_index = 0;
    for (i, c) in s.chars().enumerate() {
        if vowels.contains(&c) && vowel_index == 0 {
            vowel_index = i;
            break;
        }
    }

    let out = String::from(&s[vowel_index..]);
    match vowel_index {
        0 => format!("{out}-hay"),
        _ => format!("{out}-{}ay", &s[0..vowel_index]),
    }
}

fn summary_3() {
    use std::collections::HashMap;

    fn add_name_to_department(
        company: &mut HashMap<String, Vec<String>>,
        department: String,
        name: String,
    ) {
        let names = company.entry(department).or_insert(vec![]);
        names.push(name);
        names.sort()
    }

    // HashMap<Department, Vec[Names]>
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    add_name_to_department(
        &mut company,
        String::from("Sales"),
        String::from("Bob White"),
    );
    add_name_to_department(
        &mut company,
        String::from("Sales"),
        String::from("Joey Guy"),
    );
    add_name_to_department(
        &mut company,
        String::from("Sales"),
        String::from("Bozo the Clown"),
    );
    add_name_to_department(
        &mut company,
        String::from("Programming"),
        String::from("Amanda"),
    );

    println!("{:?}", company);
}
