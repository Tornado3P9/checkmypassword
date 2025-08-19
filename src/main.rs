use reqwest::{self, Response};
use sha1::digest::core_api::CoreWrapper;
use sha1::{Digest, Sha1, Sha1Core};
use std::io::Write;
use std::io;
use std::error::Error;

async fn request_api_data(query_char: &str) -> Result<String, Box<dyn Error>> {
    let url: String = format!("https://api.pwnedpasswords.com/range/{}", query_char);
    let res: Response = reqwest::get(&url).await?;
    if !res.status().is_success() {
        return Err(format!("Error fetching: {}, check the API and try again", res.status()).into());
    }
    Ok(res.text().await?)
}

fn get_password_leaks_count(hashes: &str, hash_to_check: &str) -> u32 {
    for line in hashes.lines() {
        let mut parts = line.split(':');
        if let (Some(h), Some(count)) = (parts.next(), parts.next()) {
            if h == hash_to_check {
                return count.parse().unwrap_or(0);
            }
        }
    }
    0
}

async fn pwned_api_check(password: &str) -> Result<u32, Box<dyn Error>> {
    let mut hasher: CoreWrapper<Sha1Core> = Sha1::new();
    hasher.update(password.as_bytes());
    let sha1password = hasher.finalize();
    let sha1password_hex: String = format!("{:X}", sha1password);
    let (first5_char, tail) = sha1password_hex.split_at(5);
    let response: String = request_api_data(first5_char).await?;
    Ok(get_password_leaks_count(&response, tail))
}

fn get_user_input() -> Result<String, io::Error> {
    print!("Please enter a password(s): ");
    io::stdout().flush()?;
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    Ok(user_input.trim().to_string()) // Trim to remove any trailing newline
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input: String = get_user_input()?;
    let args: Vec<String> = input.split_whitespace().map(String::from).collect();
    for password in args {
        let count: u32 = pwned_api_check(&password).await?;
        if count > 0 {
            println!("{} was found {} times... you should probably change your password!", password, count);
        } else {
            println!("{} was NOT found. Carry on!", password);
        }
    }
    Ok(())
}
