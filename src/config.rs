use std::{fs, collections::BTreeMap};

use toml::Value;

#[derive(Debug, Clone)]
/// struct resemble the Config structure
pub struct Config {
    pub conf: toml::Value
}

impl Config {

    pub fn new(conf: toml::Value)->Config{
        Config { conf }
    }

    /// this will
    /// 1) read the config path env variable
    /// 2) check for the default file name 
    /// 3) parse file
    pub fn read() -> Config{

        // reading the env variable for the config path
        let env = std::env::var("TOMODORO_PATH").unwrap();

        // stands for string config
        let sconfig = fs::read_to_string(env.clone()+"/tomodoro.toml").unwrap();

        let conf = toml::de::from_str(sconfig.as_str()).unwrap();

        Config{
            conf
        }

    }

    /// checks wether the `self.conf` contains `key`
    fn contains(&self, key:& str) -> bool {

        if let toml::Value::Table(table) = &self.conf{
            return  table.contains_key(key);
        }

        false
        
    }

    pub fn filter(&self, v:& Vec<&str>) -> toml::Value{

        let mut toml_table = toml::Value::Table(toml::map::Map::new());

        for widget in v {
            if self.contains(widget) {
                toml_table.as_table_mut().unwrap().insert(
                    widget.to_string().clone(),
                    self.conf[widget].clone()
                );
            }
        }

        toml_table

    }

    pub fn validate() -> bool{
        todo!()
    }

    /// sorts the given `config.conf` struct field with respect to the
    /// `key` parameter
    pub fn sort_with(&self, key: String, _value_type: String) -> Vec<(String, Value)>{

        if let toml::Value::Table(table) = self.conf.clone(){

            let mut sorted_table: BTreeMap<String, toml::Value> = BTreeMap::new();

            for (key, value) in table.iter() {
                sorted_table.insert(key.clone(), value.clone());
            }

            let mut temp: Vec<(String, Value)> = sorted_table.into_iter().collect();

            let key_to_sort_by = key;

            temp.sort_by_key(|key|{
                key.1.get(key_to_sort_by.clone()).unwrap().as_integer()
            });

            return temp;

        }

        Vec::new()

    }

}

mod Test{

    
    
    use toml::{self};

    

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
    fn contains_test() {

        let tconfig = toml::de::from_str(r#"
            [Timer]
              color = '#000000'
              width = 0.5
              height = 0.4
              x = 20
              y = 40
        "#).unwrap();

        let conf = Config {
            conf: tconfig,
        };

        let mut key = "thing";
        assert_eq!(false, conf.contains(key));
        key = "Timer";
        assert_eq!(true, conf.contains(key));
        key="";
        assert_eq!(false, conf.contains(key));

    }

    #[test]
    fn filter_test() {

        let tconfig = toml::de::from_str(r#"
            [Default]
              value = "Timer"
            [Timer]
              color = '#000000'
              width = 0.5
              height = 0.4
              x = 20
              y = 40
            [Button]
              color = '#000000'
              width = 0.5
              height = 0.4
              x = 20
              y = 40
            
        "#).unwrap();

        let conf = Config {
            conf: tconfig,
        };

        let to_filter = vec!["Timer", "Default"];

        let returned = conf.filter(&to_filter);

        println!("after filter:\n{}", returned);
        println!("subset: \n{}", returned[to_filter[0]]);

    }
    
    #[test]
    fn sort_test() {

        let tconfig = toml::de::from_str(r#"
            [Timer]
              color = '#000000'
              width = 0.5
              height = 0.4
              x = 10
              y = 40
            [Button]
              color = '#000000'
              width = 0.5
              height = 0.4
              x = 20
              y = 10
            [Widget]
              color = '#000000'
              width = 0.5
              height = 0.4
              x = 0
              y = 10
        "#).unwrap();

        let conf = Config {
            conf: tconfig,
        };

        let result = conf.sort_with("y".to_string(), "integer".to_string());

        assert_eq!(
            result[0].0,String::from("Button")
        );

        println!("result:\n{:?}", result[0]);

    }

}
