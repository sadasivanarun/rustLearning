pub fn is_leap_year(x:i16)->bool
{
    if get_remainder(x) == 0{
      return true;
    } else {
      return false;
    }
}

fn get_remainder(x:i16)->i16
{
  if x%100 == 0{
    (x/100)%4
  }
  else{
    x%4
  }
}
