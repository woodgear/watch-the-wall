use std::time::{Duration,Instant};
use reqwest;
use chrono::prelude::*;
#[macro_use]
extern crate clap;

#[derive(Clap,Debug)]
#[clap(version = "0.1", author = "废物点心")]
struct Opts {
    #[clap(short = "u", long = "url", default_value = "http://www.google.com")]
    url:String,
    #[clap(short = "p", long = "proxy")]
    proxy:Option<String>
}

trait TimeExt {
    fn ms(&self)->Duration;
    fn s(&self)->Duration;

}

impl TimeExt for u32 {
    fn ms(&self)->Duration {
        return Duration::from_millis(*self as u64)
    }
    fn s(&self)->Duration {
        return Duration::from_secs(*self as u64)
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("ops {:?}",opts);
    let mut t = term::stdout().unwrap();
    let url = opts.url;
    let proxy = opts.proxy;
    loop {
        let (status,spend_time) = time(2.s(),||curl(&url, &proxy));
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

fn time<F>(at_least:Duration,f:F)-> (Result<(),failure::Error>,Duration)
where F:Fn()->Result<(),failure::Error> {
    let instant = Instant::now();
    let res = f();
    let spend = instant.elapsed();
    if at_least>spend {
        std::thread::sleep(at_least-spend);
    }
    return (res,spend)
}

fn curl(url:&str,proxy:&Option<String>)->Result<(),failure::Error> {
    let  mut res = reqwest::Client::builder();
    if let Some(proxy) = proxy {
        res = res.proxy(reqwest::Proxy::all(proxy)?);
    }
    let res = res
    .timeout(Duration::from_secs(60))
    .build()?
    .get(url)
    .send()?;

    if res.status().as_u16() !=200 {
        return Err(failure::format_err!("{:?}",res))
    }
    Ok(())
}
