use rand::prelude::*;
use rand_pcg::Pcg64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_numbers() -> String {
    // Create a timestamp that will only change once a day.
    let timestamp = chrono::Utc::now().date().and_hms(0, 0, 0).timestamp();

    // Seed a RNG with the custom created timestamp.
    let mut rng = Pcg64::seed_from_u64(timestamp as u64);

    // Create 10 random numbers with our seeded RNG.
    let mut numbers = Vec::with_capacity(10);
    for idx in 0..10 {
        numbers.insert(idx, format!("{:02}", rng.gen_range(1..100)));
    }

    numbers.join(" - ")
}
