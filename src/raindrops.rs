use std::vec::Vec;

pub fn raindrops(n: u32) -> String {

    let mut results = String::from("");

    let mut holder = Vec::new();

    if n % 3 == 0 {
        holder.push(3);
    }; 
    if n % 5 == 0 {
        holder.push(5);
    };
    if n % 7 == 0 {
        holder.push(7);
    };
    if holder.is_empty() {
        holder.push(n);
    };

    for i in holder {
        match i {
            7 => results.push_str("Plong"),
            5 => results.push_str("Plang"),
            3 => results.push_str("Pling"),
            _ => results.push_str(&i.to_string())
        }

    }
    return results.to_string();
}


