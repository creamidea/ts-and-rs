struct MutStr<'a, 'b> {
  s: &'a mut &'b str
}

fn main() {
  let mut r = "hello";
  *MutStr { s: &mut r }.s = "world";
  println!("{}", r);
}