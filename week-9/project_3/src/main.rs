use std::io::Write;


fn main() {
   let commisioner = vec!["Ajigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
   let ministry = vec!["Intrernal Affairs","Justice","Defense","Power & Steel","Petroleum"];
   let zone = vec!["South West","North East","South South","South West","South East"];

   let mut record = String::new();
   for index in 0..zone.len(){
   record.push_str(
        &format!("{},{},{}\n",commisioner[index],ministry[index],zone[index]),
        )
   } 
   let mut file = std::fs::File::create("EFCC.csv").expect("Failed to create");
   file.write_all(record.as_bytes()).expect("write failed");
   println!("saved")

}
