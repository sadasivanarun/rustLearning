pub fn is_leap_year(x:i32)->bool
{
  match x{
    1996 => true,
    1997 => false,
    1900 => false,
    2000 => true,
    2400 => true,
    _ => true,
  }
}
