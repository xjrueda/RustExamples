macro_rules! vector {
   ($($x:expr),+) => ({
       let mut v = Vec::new();
       $( v.push($x); )+
       v
   });
}

macro_rules! fixMsg {
   ($($x:expr),+) => ({
       let mut s = "1=FIX4.4|".to_string();
       $(
        s.push_str("TAG=");
        s.push_str(&($x).to_string());
        s.push_str("|");
       )+
       s
   });
}
