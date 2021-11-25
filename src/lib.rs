//! Help to parse [playlist](https://en.wikipedia.org/wiki/PLS_(file_format)) files
//! and copy the corresponding music to your mobile device while maintaining music library
//! directory structure.
//!
//! This crate assumes relatives paths in your playlist file. If you need to tinker even more with
//! `.pls` files, check out this [crate](https://github.com/nabijaczleweli/pls-rs).
//!
//! Playlist files (.pls) can be read by VLC, which makes it suitable to be read on Iphone devices.

use regex::Regex;

/// Parses music filepath from `playlist_raw`. Files are parsed if they are delimited as followed:
/// `FileX=*` where 'X' is an integer
pub fn parse_playlist_files(playlist_raw: &str) -> Result<Vec<String>, ()> {
    let mut music_filepaths = vec![];
    let path_to_music_file_re: Regex = Regex::new(r"(?m)^File\d=(.+)$").unwrap();

    for cap in path_to_music_file_re.captures_iter(playlist_raw) {
        music_filepaths.push(cap[1].to_string());
    }

    Ok(music_filepaths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_music() {
        let playlist_raw = "garbage";
        let music_filepaths = parse_playlist_files(playlist_raw);
        assert!(music_filepaths.is_ok());
        let music_filepaths = music_filepaths.unwrap();
        assert_eq!(music_filepaths.len(), 0)
    }

    #[test]
    fn one_music() {
        let playlist_raw = "File1=/path/to/random";
        let music_filepaths = parse_playlist_files(playlist_raw);
        assert!(music_filepaths.is_ok());
        let music_filepaths = music_filepaths.unwrap();
        assert_eq!(music_filepaths.len(), 1);
        assert_eq!(music_filepaths.get(0).unwrap(), "/path/to/random");
    }

    #[test]
    fn many_music() {
        let playlist_raw = "File1=/path/to/random1\nFile2=/path/to/random2";
        let music_filepaths = parse_playlist_files(playlist_raw);
        assert!(music_filepaths.is_ok());
        let music_filepaths = music_filepaths.unwrap();
        assert_eq!(music_filepaths.len(), 2);
        assert_eq!(music_filepaths.get(0).unwrap(), "/path/to/random1");
        assert_eq!(music_filepaths.get(1).unwrap(), "/path/to/random2");
    }
}
