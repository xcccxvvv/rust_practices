use std::fs;

use anyhow::Result;
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Url;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Ethan Zhang")]
struct Args {
    #[clap(short, long)]
    #[clap(parse(try_from_str = parse_url))]
    url: String,

    #[clap(short, long)]
    #[clap(parse(try_from_str = parse_output))]
    output: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn parse_output(s: &str) -> Result<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.+.md$").unwrap();
    }
    if let true = RE.is_match(s) {
        Ok(s.to_string())
    } else {
        Ok("output.md".to_string())
    }
}

fn main() {
    let args = Args::parse();
    println!("Fetching url: {}", args.url);
    let body = reqwest::blocking::get(args.url).unwrap().text().unwrap();
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    println!("Converted markdown save in {}.", args.output);
    fs::write(args.output, md.as_bytes()).unwrap();
    println!("Saved...")
}
