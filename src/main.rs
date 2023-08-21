/// Rust program to get Verizon Edgecast CDN IP's from Azure API
/// Then print them in format for nginx real_ip module

// use dotenv; // used in req_get_info.rs

mod parse_cdn;
use parse_cdn::*;
mod req_get_info;
use req_get_info::*;


#[tokio::main]
async fn main() {
    let token = get_oauth_token().await.unwrap();
    println!("Token: {}", token);
    println!("\nnow get Verizon edgenode IP's!\n\n");
    let cdn_values = get_edgenode_ips(&token).await.unwrap();
    // println!("IP's {:#?}", cdn_values);
    print_nginx_uniq_config(cdn_values);
}

fn print_nginx_uniq_config(cdn_values: CdnValues) {
    let mut ip_list: Vec<String> = Vec::new();
    for edge_node in cdn_values.value {
        for ip_group in edge_node.properties.ip_address_groups {
            for ip in ip_group.ipv4_addresses {
                ip_list.push(format!("{}/{}", ip.base_ip_address, ip.prefix_length));
            }
        }
    }
    ip_list.sort();
    ip_list.dedup();
    for ip in ip_list {
        println!(
            "    set_real_ip_from  {:<20} # AS15133 - EdgeCast CDN (Verizon)",
            ip
        );
    }
}

// Query api for edgenode ip's - https://learn.microsoft.com/en-us/rest/api/cdn/edge-nodes/list?tabs=HTTP
// Authentication through auzure oauth token
// https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow

