use std::ptr::NonNull;

pub fn number_formatting() {
 
  //INFO Basi numeriche
  // Binario
  println!("{0} in formato binario è: {:b}", 10);
  // 10 in formato binario è: 1010

  // Ottale
  println!("{0} in formato ottale è: {:o}", 10);
  // 10 in formato ottale è: 12

  // Esadecimali
  println!("Red: #{:x}{:x}{:x}00", 255, 00, 00); // Red: #ff0000
  println!("Green: #0{:X}{:X}{:X}0", 00, 255, 00); // Green: #00FF00
  println!("Blue: #00{:X}{:X}{:X}", 00, 00, 255); // Blue: #0000FF


  //INFO Prefissi delle basi
  let n = 42;
  println!("{:#b}", n);    // 0b101010
  println!("{:#o}", n);    // 0o52
  println!("{:#x}", n);    // 0x2a
  println!("{:#X}", n);    // 0x2A
  
  
  //INFO Padding e allineamenti
  let n = 7;
  
  // Allineamenti
  println!("{:>4}", n);     //    7
  println!("{:<4}", n);     // 7   
  println!("{:^4}", n);     //  7  
  
  // Padding
  println!("{:05}", n);     // 00007
  println!("{:0>5}", n);    // 00007
  println!("{:0<5}", n);    // 70000
  println!("{:*<5}", n);    // 7****
  println!("{:*^5}", n);    // **7**
  println!("{:*>5}", n);    // ****7
  
  
  //INFO Parametri dinamici in formattazione
  let num = 3.1415;
  let width = 8;
  let precision = 2;
  println!("{:>width$.precision$}", num, width=width, precision=precision); // "    3.14"
  
  
  //INFO Precisione e formattazione floating-point
  const PI: f64 = std::f64::consts::PI;
  
  println!("The current value of pi is {PI:.3}");
  // Oppure
  println!("The current value of pi is {:.3}", PI);
  
  println!("{:8.3}", PI);     //    3.142 (campo largo 8, 3 decimali)
  println!("{:08.5}", PI);    // 03.14159 (padding con zeri)
  
  
  //INFO Notazione scientifica
  let n = 12345.6789;
  println!("{0} in notazione scientifica vale: {:e}", n);// 1.23456789e4
  println!("{0} in notazione scientifica vale: {:E}", n);// 1.23456789E4
  
  
  //INFO Segno e spaziatura
  let pos = 42;
  let neg = -42;
  println!("{:+}", pos);   // +42
  println!("{:+}", neg);   // -42
  println!("{: }", pos);   //  42
  println!("{: }", neg);   // -42

  
  //INFO Esempi avanzato
  let x = 255;
  let f = 12.34567;
  
  println!("dec = {0}, bin = {0:#010b}, oct = {0:#06o}, hex = {0:#04x}, HEX = {0:#04X}", x);
  // dec = 255, bin = 0b11111111, oct = 0o0377, hex = 0xff, HEX = 0xFF
  
  println!("{:>10}", f);          // allineamento a destra
  println!("{:<10.3}", f);        // allineamento a sinistra, 3 decimali
  println!("{:^12.2}", f);        // centrato, 2 decimali
  println!("{:08.2}", f);         // padding con zeri, 2 decimali
  println!("{:+.1e}", f);         // notazione scientifica, 1 decimale, sempre segno 
}