
	
use hyper::Client;
use bytes::buf::BufExt;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Data {
	origin: String,
}


#[tokio::main]
async fn main(){
		let url = "http://httpbin.org/ip".parse().unwrap();
		let res = Client::new().get(url).await.unwrap(); //  - для обработки ошибки, если не использовать match
		
		let body = hyper::body::aggregate(res).await.unwrap();
		
		let deserialized: Data = serde_json::from_reader(body.reader()).unwrap();
		println!("deserialized = {:?}", deserialized);

		
		//let ip:serde_json::Value = serde_json::from_reader(body.reader()).unwrap();
				
		//let strip = ip["origin"].as_str().unwrap();
		//println!("{}", strip);
		//println!("{}",ip)
}	