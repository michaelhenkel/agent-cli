use std::io::Write;
use agent_cli::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use agent_cli::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::cli_client::CliClient;
use tokio::sync::BarrierWaitResult;
use clap::{Args, Parser, Subcommand};




#[derive(Debug, Parser)]
#[clap(name = "agent")]
#[clap(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Get {
        kind: String,
        namespace: String,
        name: String,
    },
    #[clap(arg_required_else_help = true)]
    List {
        kind: String,
        namespace: String,
        name: String,
        to: String,
        filter: Vec<String>,
    },
}


fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::parse();


    let mut client = CliClient::connect("http://[::1]:50051").await?;

    match args.command {
        Commands::Get { kind, namespace, name } => {
            let request = tonic::Request::new(v1::Key { name: name.to_string(), namespace: namespace.to_string(), kind: kind.to_string() });
            let response = client.get(request).await?;
            protobuf_json_mapping::print_to_string(response);
            println!("RESPONSE={:?}", response);
        }
        Commands::List { kind, namespace, name, to, filter } => {
            let from_key = v1::Key{
                kind: kind.to_string(),
                namespace: namespace.to_string(),
                name: name.to_string(),
            };
            let to_key = v1::Key{
                kind: to.to_string(),
                namespace: "".to_string(),
                name: "".to_string(),
            };
            let mut filter_list: Vec<String> = Vec::new();
            for f in filter{
                filter_list.push(f);
            }

            let request = tonic::Request::new(v1::FromToFilter{
                from: Some(from_key),
                to: Some(to_key),
                filter: filter_list,
            });
            let response = client.find(request).await?;
            println!("RESPONSE={:?}", response);
        }
    };


    /* 
    loop {
        let input=prompt("> ");
        match input.as_str(){
            "get" => {
                let request = tonic::Request::new(v1::Key { name:"default-podnetwork".to_string(), namespace: "contrail-k8s-kubemanager-cluster1-local-contrail".to_string(), kind: "VirtualNetwork".to_string() });
                let response = client.get(request).await?;
                println!("RESPONSE={:?}", response);
            },
            "list" => {
                let from_key = v1::Key{
                    kind: "VirtualMachine".to_string(),
                    namespace: "k8splanner-system".to_string(),
                    name: "k8splanner-controller-manager-5d4db57d68-g9fvj".to_string(),
                };
                let to_key = v1::Key{
                    kind: "VirtualNetwork".to_string(),
                    namespace: "".to_string(),
                    name: "".to_string(),
                };
                let filter = vec!["VirtualNetwork".to_string(),"VirtualMachineInterface".to_string()];
                let request = tonic::Request::new(v1::FromToFilter{
                    from: Some(from_key),
                    to: Some(to_key),
                    filter: filter,
                });
                let response = client.find(request).await?;
                println!("RESPONSE={:?}", response);
                
            },
            "exit" => { break; },
            _ => {},
        };


        
        /* 
        if input=="now" {
            let unixtime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
            print!("Current Unix time is {:?}\n", unixtime);
        } 
        */

    }
    */
    Ok(())
}