fn main() {
    println!("Running ObliQuery test entrypoint!");
    // Call some core logic here for quick testing
    obq_core::plan::parse_sql("SELECT * FROM lineitem;");
}