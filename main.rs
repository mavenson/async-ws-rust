use std::env;
use std::error::Error;
use tungstenite::{Message, connect};
use url::Url;
use rust_decimal::Decimal; 
use rust_decimal_macros::*; 
use serde::{Serialize, Deserialize};
use serde_json::{json, Result, Value};
use tokio::prelude::*;
use redis::Commands;



macro_rules! rd_conn {
    () => {
	{
	    fn new_con() -> redis::RedisResult<redis::Connection> {
		let client = redis::Client::open("redis://127.0.0.1/")?;
		let result = client.get_connection()?;
		Ok(result)
	    }
	    &mut new_con().unwrap()
	}
    };
}

macro_rules! rd_args {
    ($args:expr) => {
	{
	    fn mk_cmd(args: &[&str]) -> redis::RedisResult<redis::Cmd> {
		let mut result = redis::cmd(args[0]);
		for e in args[1..].iter() {
		    result.arg(*e);
		}
		Ok(result)
	    }
	    mk_cmd($args).unwrap()
	}
    };
}

macro_rules! rd_cmd_re {
    ($type:path, $cmd:expr)  => {
	{
	    fn gen_cmd(args: &[&str]) -> redis::RedisResult<$type> {
		let mut con = rd_conn!();
		let mut rd_cmd = rd_args!(args);
		let result: $type = rd_cmd.query(con).unwrap();
		Ok(result)
	    }
	    gen_cmd($cmd).unwrap()
	}
    };
}

macro_rules! rd_cmd_no_re {
    ($cmd:expr) => {
	{
	    fn gen_cmd(args:&[&str]) -> redis::RedisResult<()> {
		let con = rd_conn!();
		let rd_cmd = rd_args!(args);
		let _ : () = rd_cmd.query(con).unwrap();
		Ok(())
	    }
	    gen_cmd($cmd).unwrap()
	}
    };
}

macro_rules! arr_to_struct {
    (name:ident, $(arg:expr),*) => {
	struct $name {
	    $(
		$arg: String,
	    )*
	}
    };
}

macro_rules! struct_to_arr {
    (strct:tt, fields:tt) => {
	{
	let mut result = Vec::new(String);
	for e in $fields.iter() {
	    result.push($strct.&e);
	}
	    result
	}
    };
}


#[tokio::main]
async fn main() -> () {
tokio::spawn(async move {
    launch_ws("subscribe", "BTC-USD", "ticker").await;
});
tokio::spawn(async move {
    launch_ws("subscribe", "ETH-USD", "ticker").await;
}).await;
}

#[derive(Serialize, Deserialize, Debug)]


struct HandShake {
    price: String,
    time: String,
}


async fn launch_ws(typ: &str, pids: &str, chnl: &str) {
    let request = gen_req_msg(typ, pids, chnl);
    let (mut socket, _) =
	connect(url::Url::parse("wss://ws-feed.pro.coinbase.com").unwrap()).
	expect("Can't connect");
    socket.write_message(Message::Text(request.into())).unwrap();
    let pl = &mut socket.read_message().unwrap();
    arr_to_struct!(HandShake.parse::<ident>(), "channels");
   let msg: HandShake = serde_json::from_str(&pl.to_string()).unwrap();
    let pl_len =  &pl.len();
    println!("{:?}", &msg);
    println!("{:?}", pl_len);
    if pl_len > &0 {
	println!("Connection to {} Successful!", &pl);
    } else {
	let pl = &mut socket.read_message().unwrap();
    }
    //let pl = tungstenite::Message::into_text(socket.read_message().unwrap()).unwrap();
    //		assert_eq!(*&pl.len(), 81);
    loop{
	let pl = &mut socket.read_message().unwrap();
	let msg: HandShake = serde_json::from_str(&pl.to_string()).unwrap();
	//		rd_cmd_no_re!(&["XADD", "btc_and_eth", "*", "msg", &pl.to_string()]);
	println!("{} ... {}", &msg.price, &msg.time);
    }
}


fn gen_req_msg(typ: &str, prd_ids: &str, chnls: &str) -> String {
	let req_msg_typ = String::from(r#"{"type": ""#);
	let req_msg_prd_ids = String::from(r#"", "product_ids": [""#);
	let req_msg_chnls = String::from(r#""], "channels": [""#);
	let req_msg_close = String::from(r#""]}"#);
	let result = format!("{}{}{}{}{}{}{}",
			     &req_msg_typ,&typ,
			     &req_msg_prd_ids,&prd_ids,
			     &req_msg_chnls,&chnls,&req_msg_close);
	println!("{}", result);
	    result
}



