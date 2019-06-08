pub fn run() {
  // Default : i32
  let _x = 2;

  // Default: f64
  let _y = 2.5;

  // Add explicit type

  let _z: i64 = 34448485484;

  // Find max size

  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);
  println!("Max f64: {}", std::f64::MAX);
  println!("Max f32: {}", std::f32::MAX);

  let is_active = true;

  let is_greater: bool = 10 < 5;
  let a1 = 'a';
  let face = '\u{1F600}';
  println!("{:?}", (_x, _y, _z, is_active, is_greater, a1, face));

}
