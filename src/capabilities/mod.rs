use tui::layout::Rect;

/// checks if sr "small rectangle" is contained inside br "big rectangle" 
pub fn compare_rect(br:& Rect, sr:& Rect) -> Result<(),&'static str>{

    let bw = br.x + br.width;
    let sw = sr.x + sr.width;
    let bh = br.y + br.height;
    let sh = sr.y + sr.height;

    // width difference and hight difference respectively
    let wd: i32 = bw as i32 - sw as i32;
    println!("{}", wd);
    let hd: i32 = bh as i32 - sh as i32;
    println!("{}", hd);

    if wd < 0 {
        Err("widget width is out of scope")
    }else if hd < 0{
        Err("widget height is out of scope")
    } else {
        Ok(())
    }

}
