use std::time::Duration;

use tui::layout::Rect;

/// checks if sr "small rectangle" is contained inside br "big rectangle" 
pub fn compare_rect(br:& Rect, sr:& Rect) -> Result<(),&'static str>{

    let bw = br.x + br.width;
    let sw = sr.x + sr.width;
    let bh = br.y + br.height;
    let sh = sr.y + sr.height;

    // width difference and hight difference respectively
    let wd: i32 = bw as i32 - sw as i32;
    // println!("{}", wd);
    let hd: i32 = bh as i32 - sh as i32;
    // println!("{}", hd);

    if wd < 0 {
        Err("widget width is out of scope")
    }else if hd < 0{
        Err("widget height is out of scope")
    } else {
        Ok(())
    }

}

/// Converts the `duration: Duration` parameter to a mm:ss string format
pub fn time_conversion(duration: Duration) -> String {
    let s_duration = duration.as_secs();
    let mut minutes:String = (s_duration / 60).to_string();
    let mut seconds:String = (s_duration % 60).to_string();
    if minutes.len() < 2 {
        minutes.insert(0, '0');
    }
    if seconds.len() < 2 {
        seconds.insert(0, '0');
    }
    let res = format!("{}:{}",minutes, seconds);
    res
}

/// converts a string of color, e.g "#00ffaa", "#aabbcc", ...\
/// into their basic rgb colors as a tuple of 3 u8's as\
/// `Option<(u8, u8, u8)>`.
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return None; // Invalid format
    }

    let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
    let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
    let b = u8::from_str_radix(&hex[5..7], 16).ok()?;

    Some((r, g, b))
}

/// Checks if a string is a number, meaning a signed integer\
/// returns `true` if the provided string is an integer\
/// and `false` if not.
pub fn is_number(string: &str) -> bool{

    match string.parse::<i64>() {
        Ok(_) => true,
        Err(_) => false
    }

}

/// Checks if a string is a float\
/// returns `true` if the provided string is an integer\
/// and `false` if not.
pub fn is_float(string: &str) -> bool{

    match string.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false
    }

}


mod Test{
    use std::{time::Duration, thread::sleep};

    use super::time_conversion;

    use std::time::SystemTime;

    #[test]
    fn time_conversion_succeed() {

        assert_eq!("00:00".to_string(), time_conversion(Duration::from_millis(10)));
        assert_eq!("00:10".to_string(), time_conversion(Duration::from_secs(10)));
        assert_eq!("01:40".to_string(), time_conversion(Duration::from_secs(100)));
        assert_eq!("25:00".to_string(), time_conversion(Duration::from_secs(1500)));
    }

    #[test]
    fn test2() {

        let sys_time = SystemTime::now();
        sleep(Duration::from_secs(1));
        let new_sys_time = SystemTime::now();
        let difference = new_sys_time.duration_since(sys_time)
            .expect("Clock may have gone backwards");
        println!("{difference:?}");
    }

}
