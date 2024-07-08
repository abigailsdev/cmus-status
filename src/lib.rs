//! A crate for serialising cmus status program text outputs for use in programs like status bars
//! and Discord RPC programs.
#[cfg(test)]
mod tests;

/// Different status types for the structure.
#[derive(Debug, Clone, PartialEq)]
pub enum STATUSTYPE {
    /// File is playing.
    PLAYING,
    /// File is paused.
    PAUSED,
    /// Player is stopped.
    STOPPED,
    /// Player is exiting.
    EXITING,
    /// Unknown status.
    UNDEFINED,
}

/// Structured cmus status data.
#[derive(Debug, Clone, PartialEq)]
pub struct Status {
    /// Status type.
    pub status: STATUSTYPE,
    /// File location.
    pub file: Option<String>,
    /// Track artist name.
    pub artist: Option<String>,
    /// Album artist name.
    pub albumartist: Option<String>,
    /// Album name.
    pub album: Option<String>,
    /// Disc number.
    pub discnumber: Option<u32>,
    /// Track number.
    pub tracknumber: Option<u32>,
    /// Song title.
    pub title: Option<String>,
    /// Album release year.
    pub date: Option<u32>,
    /// Duration of song in seconds.
    pub duration: Option<u32>,
}

impl Status {
    /// Creates and returns an empty `Status` object.
    pub fn new() -> Status {
        return Status { status: STATUSTYPE::UNDEFINED, file: None, artist: None, albumartist: None, album: None, discnumber: None, tracknumber: None, title: None, date: None, duration: None };
    }
}

/// Takes in raw cmus status data from a status program and turns it into structured data for use
/// in Rust programs.
pub fn serialise(status: &str) -> Status {
    let mut modstatus = status.to_string();
    modstatus = modstatus.replace("status ", ";;st:");
    modstatus = modstatus.replace("file ", ";;fi:");
    modstatus = modstatus.replace("albumartist ", ";;ab:");
    modstatus = modstatus.replace("artist ", ";;ar:");
    modstatus = modstatus.replace("album ", ";;al:");
    modstatus = modstatus.replace("discnumber ", ";;dn:");
    modstatus = modstatus.replace("tracknumber ", ";;tn:");
    modstatus = modstatus.replace("title ", ";;ti:");
    modstatus = modstatus.replace("date ", ";;da:");
    modstatus = modstatus.replace("duration ", ";;du:");

    let segments: Vec<String> = modstatus.split_whitespace().map(|x| x.to_string()).collect();


    let mut status = Status::new();
    let mut multi_line_file_name = false;
    let mut segtype_multi = "".to_string();
    let mut multi_line_scratch = "".to_string();
    let mut i = 0;
    loop {
        let x;
        if i >= segments.len() {
            x = "".to_string();
        } else {
            x = segments[i].clone();
        }

        if !multi_line_file_name && x.starts_with(";;") {
            let (first, last) = x.split_at(5);
            let segtype = first.replace(";;", "").replace(":", "");

            if segtype == "st" {
                match last {
                    "playing" => status.status = STATUSTYPE::PLAYING,
                    "paused" => status.status = STATUSTYPE::PAUSED,
                    "stopped" => status.status = STATUSTYPE::STOPPED,
                    "exiting" => status.status = STATUSTYPE::EXITING,
                    _ => status.status = STATUSTYPE::UNDEFINED,
                }
            } else if segtype == "fi" || segtype == "ar" || segtype == "ab" || segtype == "al" || segtype == "ti" {
                multi_line_scratch = last.to_string();
                multi_line_file_name = true;
                segtype_multi = segtype.to_string();
            } else if segtype == "dn" {
                let parsed = last.parse::<u32>();

                if parsed.is_ok() {
                    status.discnumber = Some(parsed.unwrap().clone());
                }
            } else if segtype == "tn" {
                let parsed = last.parse::<u32>();

                if parsed.is_ok() {
                    status.tracknumber = Some(parsed.unwrap().clone());
                }
            } else if segtype == "da" {
                let parsed = last.parse::<u32>();

                if parsed.is_ok() {
                    status.date = Some(parsed.unwrap().clone());
                }
            } else if segtype == "du" {
                let parsed = last.parse::<u32>();

                if parsed.is_ok() {
                    status.duration = Some(parsed.unwrap().clone());
                }
            }
            i = i + 1;
            if i >= segments.len() {
                break;
            }
        } else if multi_line_file_name {
            if !x.starts_with(";;") && x != "".to_string() {
                multi_line_scratch = format!("{} {}", multi_line_scratch, x.to_string());
                i = i + 1;
            } else {
                multi_line_file_name = false;
                match segtype_multi.as_str() {
                    "fi" => status.file = Some(multi_line_scratch.clone()),
                    "ar" => status.artist = Some(multi_line_scratch.clone()),
                    "ab" => status.albumartist = Some(multi_line_scratch.clone()),
                    "al" => status.album = Some(multi_line_scratch.clone()),
                    "ti" => status.title = Some(multi_line_scratch.clone()),
                    _ => panic!(),
                }
            }
        }
    }

    return status;
}

