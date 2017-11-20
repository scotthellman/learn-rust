use std::collections::HashMap;

fn sort(v: &mut Vec<i32>) {
    for i in 0..(v.len()) {
        let mut smallest = v[i];
        let mut smallest_idx = i;
        for j in (i+1)..v.len() {
            if smallest > v[j] {
                smallest = v[j];
                smallest_idx = j;
            }
        }
        if smallest_idx != i {
            v[smallest_idx] = v[i];
            v[i] = smallest;
        }
    }
}

fn median(v: &Vec<i32>) -> f32 {
    let mut v = v.clone();
    sort(&mut v);
    let half = v.len() as f32 / 2.0;
    (v[half.floor() as usize] + v[half.ceil() as usize]) as f32 / 2.0
}

fn mean(v: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for element in v.iter(){
        sum += *element;
    }
    sum as f32 / v.len() as f32
}

fn mode(v: &Vec<i32>) -> Option<i32> {
    let mut hits = HashMap::new();
    for element in v.iter() {
        let hit = hits.entry(*element).or_insert(0);
        *hit += 1;
    }
    let mut highest_val : Option<i32> = None;
    let mut highest_count : Option<i32> = None;
    for (value, count) in hits.iter() {
        if let Some(c) = highest_count {
            if *count <= c {
                continue;
            }
        }
        highest_count = Some(*count);
        highest_val = Some(*value);
    }
    highest_val
}

fn main() {
    let mut v = vec![2,5,8,11,15,20,4,1,20];
    println!("Mean is {}", mean(&v));
    let mo = mode(&v);
    println!("Mode is {:?}", mo);
    sort(&mut v);
    println!("Our sorted vector is {:?}", v);
    println!("Median is {}", median(&v));
}
