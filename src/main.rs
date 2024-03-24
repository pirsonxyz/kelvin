use rand::{thread_rng, Rng};

fn main() {
    let length = 15;

    let pass = generate_password(length);

    println!("The password is {}", pass);
}

fn generate_password(length: usize) -> String {
    let ascii_chars: Vec<char> = (33..=126).map(|c| c as u8 as char).collect();
    let mut rng = thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..ascii_chars.len());
            ascii_chars[idx]
        })
        .collect()
}
