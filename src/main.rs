use std::env;
use std::fs::File;
use std::io::Write;
use hidapi::HidApi;

const CODE_HUM : u8 = 0x41;
const CODE_TAMB : u8 = 0x42; /* Ambient Temperature */
const CODE_CNTR : u8 = 0x50; /* Relative Concentration of CO2 */


fn decode_temperature(w: u16) -> f64 {
    return w as f64 * 0.0625 - 273.15;
}

fn decode_humidity(w: u16) -> f64 {
    return w as f64 / 100.0;
}

//fn dump(raw: &[u8; 8]) {
//  print!("raw = ");
//  for i in 0..8 {
//    print!("0x{:02x} ", raw[i]);
//  }
//  println!();
//}

fn main() {

    // Create a temporary file.
    let temp_directory = env::temp_dir();
    let temp_file = temp_directory.join("tfaco2");

    // Open a file in write-only (ignoring errors).
    // This creates the file if it does not exist (and empty the file if it exists).
    let mut file = File::create(temp_file).expect("could not create file");



    match HidApi::new() {
        Ok(api) => {
            let device = api.open(0x04d9, 0xa052).expect("Could not open USB device");
            let magic_table : [u8; 8] = [0; 8];
            device.send_feature_report(&magic_table).expect("Could not send feature report");

            loop {
                let mut buf : [u8; 8] = [0; 8];
                device.read_timeout(&mut buf, 5000).expect("error reading usb");

                /* Check error message */
                if buf[4] != 0x0d {
                    println!("Unexpected data from device (data[4] = {:02x}, want 0x0d)", buf[4]);
                    continue;
                }

                /* Checksum */
                let r0 : u8 = buf[0];
                let r1 : u8 = buf[1];
                let r2 : u8 = buf[2];
                let r3 : u8 = buf[3];
                let checksum = 0u8
                    .wrapping_add(r0)
                    .wrapping_add(r1)
                    .wrapping_add(r2);

                if checksum != r3 {
                    println!("checksum error (0x{:02x}, await 0x{:02x})\n", checksum, r3);
                    continue;
                }

                /* Debug message */
//                dump(&result);

                /* Decode result */
                let w : u16 = ((buf[1] as u16) << 8) + buf[2] as u16;
                match r0 {
                    CODE_TAMB => {
                        let t = decode_temperature(w);
                        if writeln!(&mut file, "{{\"temp\": {t} }}").is_err() {
                            println!("error writing file");
                            continue
                        }
                    },
                    CODE_HUM => {
                        let t = decode_humidity(w);
                        if writeln!(&mut file, "{{\"hum\": {t} }}").is_err() {
                            println!("error writing file");
                            continue
                        }
                    },
                    CODE_CNTR => {
                        if writeln!(&mut file, "{{\"co2\": {w} }}").is_err() {
                            println!("error writing file");
                            continue
                        }
                    },
                    _ => { }
                }

            }

        },
        Err(e) => {
            println!("Error: {}", e);
        },
    }

}
