use std::fs;
use serde::{Serialize, Deserialize, de::{self, Visitor}};

#[derive(Debug, Clone, Copy)]
pub struct RGB(pub u8, pub u8, pub u8);

//                        Deserialization
// Toml Representation -------------------> Config Structure Object
//                        Serialization
// Toml Representation <------------------- Config Structure Object
impl Serialize for RGB{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            format!("#{:02X}{:02X}{:02X}",self.0, self.1, self.2).as_str()
        )
    }
}

impl<'de> Deserialize<'de> for RGB {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Debug)]
            struct RGBVisitor;

            impl<'de> Visitor<'de> for  RGBVisitor{

                type Value=RGB;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("tuple struct RGB")
                }


                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: de::SeqAccess<'de>, 
                    {

                        let r = match seq.next_element::<u8>()?{
                            Some(value) => value,
                            None => panic!("missing read value in {:#?}", self)
                        };

                        let g = match seq.next_element()?{
                            Some(value) => value,
                            None => panic!("missing green value in {:#?}", self)
                        };

                        let b = match seq.next_element()?{
                            Some(value) => value,
                            None => panic!("missing blue value in {:#?}", self)
                        };
                        Ok(RGB(r,g,b))
                    }


                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        let num = u32::from_str_radix(&v[1..7], 16).unwrap();

                        let r:u8 = (num >> 16).try_into().unwrap();
                        let g:u8 = (num >> 8 & 65280 >> 8).try_into().unwrap();
                        let b:u8 = (num & 255).try_into().unwrap();

                        Ok(RGB(r,g,b))
                    }
                }

            deserializer.deserialize_str(RGBVisitor)
        }  
}

struct TimerDefaults;
impl TimerDefaults {
    pub fn color()->RGB{ RGB(220,190,90) }
    pub fn background_color()->RGB{ RGB(20,90,210) }
    pub fn width()->f32{0.3}
    pub fn height()->f32{0.2}
    pub fn x()->f32{0.35}
    pub fn y()->f32{0.4}
    pub fn focus_duration()->u32{1500}
    pub fn rest_duration()->u32{300}
    pub fn max_cycles()->u8{10}
    pub fn cycles()->u8{0}
    pub fn focus_alarm()->String{ String::from("") }
    pub fn rest_alarm()->String{ String::from("") }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Timer{
    #[serde(default = "TimerDefaults::color")]
    pub color: RGB,
    #[serde(default = "TimerDefaults::background_color")]
    pub background_color: RGB,
    #[serde(default = "TimerDefaults::width")]
    pub width: f32,
    #[serde(default = "TimerDefaults::height")]
    pub height: f32,
    #[serde(default = "TimerDefaults::x")]
    pub x: f32,
    #[serde(default = "TimerDefaults::y")]
    pub y: f32,
    #[serde(default = "TimerDefaults::focus_duration")]
    pub focus_duration: u32,
    #[serde(default = "TimerDefaults::rest_duration")]
    pub rest_duration: u32,
    #[serde(default = "TimerDefaults::max_cycles")]
    pub max_cycles: u8,
    #[serde(default = "TimerDefaults::cycles")]
    pub cycles: u8,
    #[serde(default = "TimerDefaults::focus_alarm")]
    pub focus_alarm: String,
    #[serde(default = "TimerDefaults::rest_alarm")]
    pub rest_alarm: String,
}

struct ButtonDefaults;
impl ButtonDefaults {
    fn color()->RGB{ RGB(200,120,130) }
    fn background_color()->RGB{ RGB(120,190,100) }
    fn width()->f32{0.3}
    fn height()->f32{0.2}
    fn x()->f32{0.35}
    fn y()->f32{0.6}
    fn focus_banner()->String{ String::from("focus") }
    fn rest_banner()->String{ String::from("rest") }
    fn pause_banner()->String{ String::from("pause") }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Button{
    #[serde(default = "ButtonDefaults::color")]
    pub color: RGB,
    #[serde(default = "ButtonDefaults::background_color")]
    pub background_color: RGB,
    #[serde(default = "ButtonDefaults::width")]
    pub width: f32,
    #[serde(default = "ButtonDefaults::height")]
    pub height: f32,
    #[serde(default = "ButtonDefaults::x")]
    pub x: f32,
    #[serde(default = "ButtonDefaults::y")]
    pub y: f32,
    #[serde(default = "ButtonDefaults::focus_banner")]
    pub focus_banner: String,
    #[serde(default = "ButtonDefaults::rest_banner")]
    pub rest_banner: String,
    #[serde(default = "ButtonDefaults::pause_banner")]
    pub pause_banner: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(rename = "Button")]
    pub button: Button,
    #[serde(rename = "Timer")]
    pub timer: Timer,
}

impl Config {

    /// this will
    /// 1) read the config path env variable
    /// 2) check for the default file name 
    /// 3) deserialize the file 
    pub fn read() -> Config{
        // reading the env variable for the config path
        let env = std::env::var("TOMODORO_PATH").unwrap();

        // stands for string config
        let sconfig = fs::read_to_string(env.clone()+"/tomodoro.toml").unwrap();

        let conf:Config = toml::de::from_str(sconfig.as_str()).unwrap();

        conf
    }
}

mod test{
    #[allow(unused_imports)]
    use crate::config::Config;

    #[test]
    fn contains_test() {
        let conf:Config = toml::de::from_str(r#"
            [Timer]
              color = '#000000'
              width = 0.5
              height = 0.4
              x = 20
              y = 40
        "#).unwrap();
        assert_eq!(conf.timer.color.0, 0);
        assert_eq!(conf.timer.color.2, 0);
        assert_eq!(conf.timer.width, 0.5);
        assert_eq!(conf.timer.y, 40.0);
    }

    #[test]
    fn filter_test() {
        let conf:Config = toml::de::from_str(r#"
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

        assert_eq!(conf.button.width, 0.5);
        assert_eq!(conf.button.x, 20.0);
        assert_eq!(conf.button.y, 40.0);
        assert_eq!(conf.button.color.1, 0);
    }
}
