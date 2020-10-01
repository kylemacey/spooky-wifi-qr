extern crate qrcode_generator;

use clap::{Arg, App};
use qrcode_generator::QrCodeEcc;

struct WifiConfig {
  ssid: String,
  encryption: String,
  password: String,
}

impl WifiConfig {
  fn from_matches(matches: &clap::ArgMatches) -> WifiConfig {
    let ssid = matches.value_of("ssid").expect("Must provide an SSID");
    let encryption = matches.value_of("encryption").unwrap_or("");
    let password = matches.value_of("password").unwrap_or("");

    WifiConfig {
      ssid: ssid.to_string(),
      encryption: encryption.to_string(),
      password: password.to_string(),
    }
  }

  fn to_qr_string(self) -> String {
    format!("WIFI:S:{};T:{};P:{};;", self.ssid, self.encryption, self.password)
  }
}

struct OutputConfig {
  true_char: String,
  false_char: String,
  resolution: u16,
}

impl OutputConfig {
  fn from_matches(matches: &clap::ArgMatches) -> OutputConfig {
    let true_char = matches.value_of("true_char").unwrap_or("💀");
    let false_char = matches.value_of("false_char").unwrap_or("⬛️");
    let resolution = matches.value_of("resolution").unwrap_or("1");

    OutputConfig {
      true_char: true_char.to_string(),
      false_char: false_char.to_string(),
      resolution: resolution.parse::<u16>().unwrap(),
    }
  }
}

fn main() {
  let matches = App::new("Spooky Wifi QR Code Generator")
        .version("0.1.0")
        .author("Kyle Macey <kylemacey@github.com>")
        .about("Generate spooky QR codes for connecting to your Wifi in the terminal")
        .arg(Arg::with_name("ssid")
                 .short("s")
                 .long("ssid")
                 .takes_value(true)
                 .help("The SSID name to connect to"))
        .arg(Arg::with_name("encryption")
                 .short("e")
                 .long("encryption")
                 .takes_value(true)
                 .help("WEP, WPA"))
        .arg(Arg::with_name("password")
                 .short("p")
                 .long("password")
                 .takes_value(true))
        .arg(Arg::with_name("true_char")
                 .short("t")
                 .long("true_char")
                 .takes_value(true)
                 .help("the true character displayed. be extra spooky"))
        .arg(Arg::with_name("false_char")
                 .short("f")
                 .long("false_char")
                 .takes_value(true)
                 .help("the false character displayed"))
        .arg(Arg::with_name("resolution")
                 .short("r")
                 .long("resolution")
                 .takes_value(true)
                 .help("the resolution of the an individual cell n*n"))
        .get_matches();



  let wifi_config = WifiConfig::from_matches(&matches);
  let output_config = OutputConfig::from_matches(&matches);

  let result: Vec<Vec<bool>> = qrcode_generator::to_matrix(
    wifi_config.to_qr_string(), QrCodeEcc::Low
  ).unwrap();

  println!(" ");
  for row in result.iter() {
    print_row(row, &output_config)
  }
  println!(" ");
}

fn print_row(row: &Vec<bool>, output_config: &OutputConfig) {
  for _ in 0..output_config.resolution {
    for cell in row.iter() {
      let val = match cell{
        &true => &output_config.true_char,
        &false => &output_config.false_char,
      };

      for _ in 0..output_config.resolution {
        print!("{}", val);
      }
    }

    println!(" ");
  }
}