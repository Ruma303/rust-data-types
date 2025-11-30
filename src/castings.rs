pub fn cast() {
    let x: u8 = 255;
    let y: u16 = x as u16;
    println!("y: {}", y);
}

pub fn miles_as() {
  let miles_away = 50;
  let miles_away_i8 = miles_away as i8;
  let miles_away_f32 = miles_away as f32;
}