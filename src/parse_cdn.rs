use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IPAddressGroup {
    delivery_region: String,
    pub ipv4_addresses: Vec<Ipv4Address>,
    ipv6_addresses: Vec<Ipv6Address>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ipv4Address {
    pub base_ip_address: String,
    pub prefix_length: u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ipv6Address {
    base_ip_address: String,
    prefix_length: u8,
}

#[derive(Deserialize, Debug)]
pub struct EdgeNode {
    id: String,
    r#type: String,
    name: String,
    pub properties: EdgeNodeProperties,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EdgeNodeProperties {
    pub ip_address_groups: Vec<IPAddressGroup>,
}

#[derive(Deserialize, Debug)]
pub struct CdnValues {
    pub value: Vec<EdgeNode>,
}
