extern crate mpd;

use mpd::Client;
use std::env;
use std::result;

type Result<T> = result::Result<T, String>;

fn usage(program: &str) {
    eprintln!("Usage: {} <mpd address> [volume] [playlist]", program);
}

fn start() -> Result<()> {
    let mut args = env::args();
    let program = args.next().expect("Program name is always present");

    if let Some(conn_address) = args.next() {
        let mut conn: Client =
            Client::connect(&conn_address).map_err(|e| format!("Connection error: {}", e))?;

        if let Some(volume_str) = args.next() {
            let volume: u8 = volume_str
                .parse()
                .map_err(|e| format!("Invalid volume: {}", e))?;
            let volume_i8: i8 = volume
                .try_into()
                .map_err(|e| format!("Volume conversion error: {}", e))?;
            conn.volume(volume_i8)
                .map_err(|e| format!("Volume setting error: {}", e))?;
        }

        if let Some(playlist_name) = args.next() {
            conn.load(&playlist_name, ..)
                .map_err(|e| format!("Playlist loading error: {}", e))?;
        }

        conn.play().map_err(|e| format!("Play error: {}", e))?;
        println!("Status: {:?}", conn.status());
        println!("Connection: {:?}", conn_address);
    } else {
        usage(&program);
        eprintln!("Please provide a connection address!");
        return Err("No connection address provided".to_string());
    }
    println!("{:?}", program);
    Ok(())
}

fn main() {
    match start() {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
