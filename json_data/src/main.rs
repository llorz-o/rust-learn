extern crate rustc_serialize;
use rustc_serialize::json;

fn main() {

    #[derive(RustcDecodable,RustcEncodable,Debug)]
    pub struct TestStruct {
        data_int:u8,
        data_str:String,
        data_vector:Vec<u8>,
    }

    let o = TestStruct {
        data_int:1, 
        data_str:"huammer".to_string(),
        data_vector:vec![2,3,4,56],
    };

    let encode = json::encode(&o).unwrap();
    println!("{}",encode);  

    let decode:TestStruct = json::decode(&encode).unwrap();

    println!("{:?}",decode);

}
