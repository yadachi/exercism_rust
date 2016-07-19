pub fn raindrops(num: usize) -> String {
    let is_pling = |num| num % 3 == 0;
    let is_plang = |num| num % 5 == 0;
    let is_plong = |num| num % 7 == 0;
    let mut drops = String::new();
    if is_pling(num) {
        drops.push_str("Pling");
    }
    if is_plang(num) {
        drops.push_str("Plang");
    }
    if is_plong(num) {
        drops.push_str("Plong");
    }
    if drops.is_empty() {
        let s = format!("{}",num);
        drops.push_str(&s);
    }
    drops
}
