pub struct FPScounter {
    time: std::time::Instant,
}
impl FPScounter {
    pub fn start() -> FPScounter {
        FPScounter {
            time: std::time::Instant::now(),
        }
    }

    pub fn frame(&mut self) -> (f64, f64) {
        let elapsed = self.time.elapsed();
        let elapsed: f64 =
            (elapsed.as_secs() as f64) + 
            (elapsed.subsec_nanos() as f64 / 1000_000_000.0)
        ;

        self.time = std::time::Instant::now();

        //println!("Frame count: {}; \tTime since last reset: {}", self.frame_count, elapsed);
        (elapsed, (1.0 / elapsed))
    }
}
