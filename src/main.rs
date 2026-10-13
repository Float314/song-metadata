use id3::{Tag, TagLike, Version};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("-- Song metadata --");

    let music_fil: String = Input::new()
        .with_prompt("Enter Song file to edit: ")
        .interact_text()?;

    // 1. Made tag mutable
    let mut tag = Tag::read_from_path(&music_fil)?;

    // 2. Added .interact_text() to each input
    let album: String = Input::new().with_prompt("Album Name? :").interact_text()?;
    let artist: String = Input::new().with_prompt("Artist? : ").interact_text()?;
    let title: String = Input::new().with_prompt("Title? (Song name): ").interact_text()?;
    let year: i32 = Input::new().with_prompt("Year? : ").interact_text()?;

    tag.set_album(album);
    tag.set_artist(artist);
    tag.set_title(title);
    tag.set_year(year);

    // 3. Used music_fil instead of undefined file_path
    tag.write_to_path(&music_fil, Version::Id3v24)?;

    println!("Operation went successfully!");
    Ok(())
}