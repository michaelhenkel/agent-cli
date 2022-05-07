use std::time::{SystemTime};
use std::io::Write;
use agent_cli::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use agent_cli::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::cli_client::CliClient;

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CliClient::connect("http://[::1]:50051").await?;
    loop {
        let input=prompt("> ");
        let request = tonic::Request::new(v1::Key { name:"default-podnetwork".to_string(), namespace: "contrail-k8s-kubemanager-cluster1-local-contrail".to_string(), kind: "VirtualNetwork".to_string() });

        let response = client.get(request).await?;

        println!("RESPONSE={:?}", response);
        /* 
        if input=="now" {
            let unixtime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
            print!("Current Unix time is {:?}\n", unixtime);
        } 
        */
        if input=="exit" { 
            break; 
        };
    }
    Ok(())
}