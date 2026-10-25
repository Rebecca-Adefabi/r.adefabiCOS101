use std::io::Write;
use std::io::Read;


fn main() {
   
   let mut file = std::fs::File::open("commisioner.csv").expect("went wrong");
   let mut commisioner = String::new();
   file.read_to_string(&mut commisioner).expect("failed");
   let commisioner = commisioner.trim().split("\r\n");
   let mut commisioner_vec:Vec<&str> = Vec::new();
   
   
   let mut file_1 = std::fs::File::open("ministry.csv").expect("went wrong");
   let mut ministry = String::new();
   file_1.read_to_string(&mut ministry).expect("failed");
   let ministry = ministry.trim().split("\r\n");
   let mut ministry_vec:Vec<&str> = Vec::new();

   let mut file_2 = std::fs::File::open("zone.csv").expect("went wrong");
   let mut zone = String::new();
   file_2.read_to_string(&mut zone).expect("failed");
   let zone = zone.trim().split("\r\n");
   let mut zone_vec:Vec<&str> = Vec::new();

   let mut file_3 = std::fs::File::create("EFCCmerge.csv").expect("Create failed");

   for x in commisioner{
    commisioner_vec.push(x);
   }
   for y in ministry{
    ministry_vec.push(y);
   }
   for z in zone{
    zone_vec.push(z);
   }

   println!("Merge Successful");

   

   for i in 0..6{
    file_3.write_all(format!("{},{},{}\n",commisioner_vec[i],ministry_vec[i],zone_vec[i])
        .as_bytes()).expect("couldn't");
   }

   
}
