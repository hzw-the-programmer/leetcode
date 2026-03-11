// 274. H-Index

#[cfg(test)]
mod tests;

pub fn h_index(mut citations: Vec<i32>) -> i32 {
    citations.sort();
    let mut h = 0;
    for &n in citations.iter().rev() {
        if n <= h {
            return h;
        }
        h += 1;
    }
    h
}
