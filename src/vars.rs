pub fn run() {
  // Vars hold prim data or ref to data
  // vars are immut by default
  // rust is block scoped

  let name = "Ryan";
  let mut age = 30;

  println!("My name is {} and I am {} years old.", name, age);

  age = 40;

  println!("My name is {} and I am {} years old.", name, age);

  // Define constant

  const ID: i32 = 001;
  println!("ID:{}", ID);

  // Assign multiple vars

  let (my_name, my_age) = ("Brad", 37);
  println!("{} is {}.", my_name, my_age);
}
