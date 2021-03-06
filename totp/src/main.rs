use clap::{App, Arg};
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha512;
use std::time::{SystemTime, UNIX_EPOCH};

fn totp(x: u64, dt: u64, k: &str, digits: u32) -> u64 {
    // get hmac output
    let mut mac = Hmac::<Sha512>::new_varkey(k.as_bytes()).expect("Couldn't create HMAC.");
    let t = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - dt)
        / x;
    mac.update(t.to_string().as_bytes());
    let out = mac.finalize().into_bytes().to_vec(); // output of HMAC-512

    // last byte's 4 low order bits
    let offset: usize = (out[out.len() - 1] & 0xf).into();

    // P = string[offset]...string[offset+3]
    let p: u64 = ((out[offset] as u64 & 0x7f) << 24)
        | ((out[offset + 1] as u64) << 16)
        | ((out[offset + 2] as u64) << 8)
        | (out[offset + 3] as u64);

    // return last 31 bits % 10^{digits}
    p % (10 as u64).pow(digits)
}

fn main() {
    let matches = App::new("TOTP")
        .version("1.0.0")
        .author("Leonid Krashanoff <leo@krashanoff.com>")
        .about("Generate a one-time-use password in accordance with RFC 6238 using HMAC-SHA-512.")
        .args(&[
            Arg::with_name("key")
                .takes_value(true)
                .required(true)
                .value_name("KEY")
                .help("Key for the one-time password."),
            Arg::with_name("x")
                .takes_value(true)
                .long("resolution")
                .short("x")
                .value_name("X0")
                .default_value("30")
                .help("Resolution of the time-based password in seconds."),
            Arg::with_name("t")
                .takes_value(true)
                .long("offset")
                .short("t")
                .value_name("T0")
                .default_value("0")
                .help("Offset from the UNIX epoch."),
            Arg::with_name("d")
                .takes_value(true)
                .long("digits")
                .short("d")
                .default_value("8")
                .help("Digits to output."),
            Arg::with_name("v")
                .takes_value(false)
                .long("verbose")
                .short("v")
                .help("More verbose output"),
        ])
        .get_matches();

    let mut x: u64 = 30;
    if let Some(res) = matches.value_of("x") {
        x = res.parse().expect("Invalid value for argument -x");
    }

    let mut t: u64 = 0;
    if let Some(offset) = matches.value_of("t") {
        t = offset.parse().expect("Invalid value for argument -t");
    }

    let mut digits: u32 = 8;
    if let Some(num) = matches.value_of("d") {
        digits = num.parse().expect("Invalid value for argument -d");
    }

    let key = matches.value_of("key").expect("A key is required!");

    if matches.is_present("v") {
        println!(
            "Starting execution at {} seconds with key '{}'.",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            key
        );
    }

    println!("{:01$}", totp(x, t, key, digits), digits as usize);
}
