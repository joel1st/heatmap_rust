use std::cmp;

pub fn get_stamp(stamp: &String, diameter: i32) -> Vec<Vec<u8>> {
    match stamp.as_ref() {
        "circle_discrete" => circle_discrete(diameter),
        "circle" => circle(diameter),
        _ => circle_discrete(diameter)
    }
}

pub fn circle_discrete(diamator: i32) -> Vec<Vec<u8>> {
  let center: i32 = diamator / 2;
  let radius2: i32 = center.pow(2);
  println!("here");

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
	  stamp.push(pixels);
  }
  stamp
}

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
        if dx2 + dy2 <= radius2 {
            pixels.push(((cmp::max(dx.abs(), dy.abs()) * -1) + center) as u8);
        } else {
            pixels.push(0)
        }
	  }
	  stamp.push(pixels);
  }
  stamp
}
