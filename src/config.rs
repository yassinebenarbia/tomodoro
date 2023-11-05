use std::{fs, collections::HashMap};

use json::JsonValue;

use crate::displayable::Displayable;

#[derive(Debug, Clone)]
/// struct resemble the Config structure
pub struct Config {
    pub conf: JsonValue
}

impl Config {

    /// this will
    /// 1) read the config path env variable
    /// 2) check for the default file name 
    /// 3) parse file
    pub fn read() -> Config{

        let mut env = std::env::var("TOMODORO_PATH").unwrap();

        let sconfig = fs::read_to_string(env.clone()+"/tomodoro.json").unwrap();

        let conf = json::parse(sconfig.as_str()).unwrap();

        Config{
            conf
        }

    }

    pub fn filter(&self, v: Vec<&str>) -> HashMap<String, JsonValue>{

        let mut toreturn = HashMap::new();

        for widget in v {

            if self.conf.has_key(widget) {

                toreturn.insert(widget.to_string(), self.conf[widget].clone());
                
            }
            
        }

        toreturn

    }

    pub fn validate() -> bool{
        todo!()
    }

    pub fn sort_with(key: String){

    }

}
mod Test{
    use std::{fs, collections::HashMap};
    use json::JsonValue::{self, Null};

    #[test]
    fn should_work() {

        let mut env = std::env::var("TOMODORO_PATH").unwrap();

        let sconfig = fs::read_to_string(env.clone()+"/tomodoro.json").unwrap();

        let mut jconfig = json::parse(sconfig.as_str()).unwrap();
        
        let state = jconfig.contains(JsonValue::String("Timer".to_string()));

        for value in jconfig["Timer"].entries() {
            println!("{}", value.0);
            println!("{}", value.1);
        }

        println!("{}",jconfig.has_key("Timer"));


        println!("config structure:\n{}", jconfig["Timer"]);

        println!("{:}", state);

    }

    #[test]
    fn filter_test() {

        let env = std::env::var("TOMODORO_PATH").unwrap();

        let sconfig = fs::read_to_string(env+"/tomodoro.json").unwrap();

        let jconfig = json::parse(sconfig.as_str()).unwrap();
        
        let mut after_filter = HashMap::new();

        let filter = vec!["Timer", "default"];

        for value in filter.iter() {

            if jconfig.has_key(value) {

                after_filter.insert(value.to_string(), jconfig[value.to_string()].to_string());

            }
            
        }

        let temp = after_filter
            .get("Timer")
            .unwrap();

        
        println!("{}",json::parse(temp).unwrap()["x"]);

        println!("after filter\n{:?}", after_filter);

    }

}
