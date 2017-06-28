pub fn circle(diamator: i32) -> Vec<Vec<u8>> {
  let center: i32 = diamator / 2;
  let radius2: i32 = center.pow(2);

  let mut stamp = vec![];
  for y in 0..diamator {
      let mut pixels = vec![];
	  for x in 0..diamator {
		let dx: i32 = x as i32 - center;
		let dy: i32 = y as i32 - center;
		let dx2: i32 = dx.pow(2);
		let dy2: i32 = dy.pow(2);
		pixels.push((dx2 + dy2 <= radius2) as u8);
	  }
    //   println!("{:?}", pixels);
	  stamp.push(pixels);
  }
  stamp
}
