use cpal::Sample;
fn test_it<T: Sample>(val: T) -> f32 { val.to_f32() }
fn main() {}
