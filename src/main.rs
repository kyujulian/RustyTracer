fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    tracer::final_scene();
}
