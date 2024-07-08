mod tests {
    use crate::*;

    #[test]
    fn paused_file() {
        let string = "status paused file /music/song.mp3 artist Bob Acri albumartist Various Artists album Granada Full Moon, Chill, Ambient, Lounge discnumber 1 tracknumber 3 title Sleep Away date 2017 duration 200";
        let status = serialise(string);
        assert_eq!(status.status, STATUSTYPE::PAUSED);
        assert_eq!(status.file, Some("/music/song.mp3".to_string()));
        assert_eq!(status.artist, Some("Bob Acri".to_string()));
        assert_eq!(status.albumartist, Some("Various Artists".to_string()));
        assert_eq!(status.album, Some("Granada Full Moon, Chill, Ambient, Lounge".to_string()));
        assert_eq!(status.discnumber, Some(1));
        assert_eq!(status.tracknumber, Some(3));
        assert_eq!(status.title, Some("Sleep Away".to_string()));
        assert_eq!(status.date, Some(2017));
        assert_eq!(status.duration, Some(200));
    }

    #[test]
    fn playing_file() {
        let string = "status playing file /music/song.mp3 artist Bob Acri albumartist Various Artists album Granada Full Moon, Chill, Ambient, Lounge discnumber 1 tracknumber 3 title Sleep Away date 2017 duration 200";
        let status = serialise(string);
        assert_eq!(status.status, STATUSTYPE::PLAYING);
        assert_eq!(status.file, Some("/music/song.mp3".to_string()));
        assert_eq!(status.artist, Some("Bob Acri".to_string()));
        assert_eq!(status.albumartist, Some("Various Artists".to_string()));
        assert_eq!(status.album, Some("Granada Full Moon, Chill, Ambient, Lounge".to_string()));
        assert_eq!(status.discnumber, Some(1));
        assert_eq!(status.tracknumber, Some(3));
        assert_eq!(status.title, Some("Sleep Away".to_string()));
        assert_eq!(status.date, Some(2017));
        assert_eq!(status.duration, Some(200));
    }

    #[test]
    fn stopped() {
        let string = "status stopped";
        let status = serialise(string);
        let mut test_case = Status::new();
        test_case.status = STATUSTYPE::STOPPED;
        assert_eq!(status, test_case);
    }
}
