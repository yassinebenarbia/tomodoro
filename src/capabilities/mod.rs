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

/// Converts a Duration typed value to a mm:ss string format
///
/// this should output the time in the following form mm:ss
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
