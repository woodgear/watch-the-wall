use std::time::{Duration,Instant};
use reqwest;
use chrono::prelude::*;


fn main() {
    let mut t = term::stdout().unwrap();
    loop {
        let (status,spend_time) = time(curl_google);
        if status.is_ok() {
            t.fg(term::color::GREEN).unwrap();
            writeln!(t, "now {:?} success spend_time {:?}",now_str(),spend_time).unwrap();
        } else {
            t.fg(term::color::RED).unwrap();
            writeln!(t,"now {:?} success spend_time {:?}  status {:?}",now_str(),spend_time,status).unwrap();    
        }
    }
}

fn now_str()->String {
    Local::now().to_rfc2822()
}

fn time<F>(f:F)-> (Result<(),failure::Error>,Duration)
where F:Fn()->Result<(),failure::Error> {
    let instant = Instant::now();
    let res = f();
    return (res,instant.elapsed())
}

fn curl_google()->Result<(),failure::Error> {
    let res = reqwest::Client::builder()
    .proxy(reqwest::Proxy::all("http://127.0.0.1:18080")?)
    .timeout(Duration::from_secs(60))
    .build()?
    .get("http://google.com")
    .send()?;

    if res.status().as_u16() !=200 {
        return Err(failure::format_err!("{:?}",res))
    }
    Ok(())
}

#[test]
fn test_curl_google() {
    let res = curl_google();
    println!("res {:?}",res);
}