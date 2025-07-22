mod gen_dummy_files;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        gen_dummy_files::generate().unwrap();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
