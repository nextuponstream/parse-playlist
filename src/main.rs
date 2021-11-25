use clap::{crate_authors, crate_version, App, Arg};
use log::debug;
use parse_playlist::parse_playlist_files;
//use simplelog::*; // NOTE: for debugging
use std::{fs::read_to_string, ops::Add, process::exit};

fn main() -> Result<(), std::io::Error> {
    // NOTE: for debugging
    //let _ = TermLogger::init(
    //    LevelFilter::Debug,
    //    Config::default(),
    //    TerminalMode::Mixed,
    //    ColorChoice::Auto,
    //);

    let app = App::new("parse-playlist")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Parse music files from source to destination while conserving directory structure.")
        .arg(
            Arg::with_name("playlist-file")
                .short("p")
                .long("playlist-file")
                .help("The playlist file you want to parse")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("src")
                .help("the source folder where to copy music file from")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("destination")
                .short("d")
                .long("dest")
                .help("the folder to copy music files to")
                .takes_value(true)
                .required(true),
        )
        .arg(Arg::with_name("verbose").long("verbose"));
    let matches = app.get_matches();
    let src = matches.value_of("source").unwrap();
    let dest = matches.value_of("destination").unwrap();
    let verbose = matches.is_present("verbose");
    let playlist_file = matches.value_of("playlist-file").unwrap();

    debug!("playlist file: {}", playlist_file);
    let playlist_raw = read_to_string(playlist_file).unwrap();

    let music_filepaths = parse_playlist_files(&playlist_raw).unwrap();

    if music_filepaths.is_empty() {
        eprintln!("No music files to transfer. Exiting.");
        exit(1);
    }

    println!(
        "Copying {} musics from \"{}\" to \"{}\"",
        music_filepaths.len(),
        src,
        dest
    );

    for music_filepath in music_filepaths {
        if verbose {
            println!("Copying \"{}\"", music_filepath);
        }
        let mut src_file = src.to_string();
        src_file.push('/');
        let src_file = src_file.add(music_filepath.as_str());
        if verbose {
            println!("from:\t\"{}\"", src_file);
        }

        let mut dest_file = dest.to_string();
        dest_file.push('/');
        let dest_file = dest_file.add(music_filepath.as_str());
        if verbose {
            println!("to:\t\"{}\"", dest_file);
        }

        let mut dest_folder = dest_file.clone();
        if dest_folder.contains('/') {
            let mut is_folder = false;
            while !is_folder {
                is_folder = dest_folder.pop().unwrap() == '/';
            }
        }
        debug!("dest_folder: {}", dest_folder);
        if verbose {
            println!("Creating destination folder: \"{}\"", dest_folder);
        }
        std::fs::create_dir_all(dest_folder)?;
        debug!("src_file: {}", src_file);
        debug!("dest_file: {}", dest_file);
        std::fs::copy(src_file, dest_file)?;
        if verbose {
            println!("Ok");
        }
    }

    println!("All done.");

    Ok(())
}
