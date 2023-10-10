use mx_plug_core::{PluginContext, Void};
use mx_plugin_demo_shared::{Test1Args, Test1Result};

use reqwest::blocking as req;
use serde::Deserialize;

mx_plug_core::plugin! {
    name = "demo",
    fns = [test_1, test_2, test_3]
}

fn test_1(ctx: &PluginContext, args: Test1Args) -> Test1Result {
    println!("smart contract address: {:?}", ctx.address);
    return Test1Result {
        sum: args.a + args.b,
    };
}

fn test_2(_: &PluginContext, a: u8) -> u8 {
    return a * 2;
}

#[derive(Deserialize, Debug, Clone)]
struct ApiUser {
    id: u32,
    name: String,
}

fn test_3(_: &PluginContext, _: Void) -> String {
    let resp = req::get("https://jsonplaceholder.typicode.com/users").unwrap();
    let users: Vec<ApiUser> = serde_json::from_str(&resp.text().unwrap().as_str()).unwrap();
    let user = users.get(0).unwrap();
    println!("User: {:?}", user);
    return format!("{}: {}", user.id, user.name);
}
