use teloxide::{ Bot, prelude::Requester, net::Download };
use teloxide::types::{
    Message, 
    MessageKind, MediaKind, 
    MediaDocument, MediaPhoto, MediaVideo, MediaAudio, MediaVoice, 
    FileMeta, 
};
use tokio::fs::File;
use anyhow::Context; 

use std::path::PathBuf;


pub async fn handle_message(bot: Bot, message: Message, download_path: PathBuf) -> anyhow::Result<()> {
    let chat_id = message.chat.id;

    match get_file_info_from_message(&message) {
        Some((file_meta, original_file_name)) => {
            let file_name = sanitize_filename(original_file_name.clone()); 
            let final_save_path = download_path.join(&file_name);
            log::info!("Attempting to download file: '{}'", file_name);

            let file = bot.get_file(file_meta.id.clone()).await
                .context(format!("Failed to get file info from Telegram for file_id: {}", file_meta.id))?;

            let file_path_on_server = file.path;
           
            let mut out_file = File::create(&final_save_path).await
                .context(format!("Failed to create local file at {:?}", final_save_path))?;

            bot.download_file(&file_path_on_server, &mut out_file).await
                .context(format!("Failed to download file '{}' from Telegram", original_file_name))?;

            log::info!("File '{}' saved successfully to: {:?}", file_name, final_save_path);
            bot.send_message(chat_id, format!("File '{}' saved successfully!", file_name)).await?;
        }
        None => {
            if message.text().is_some() {
                bot.send_message(chat_id, "Please send me a file (document, photo, video, audio, or voice message). I received a text message or a type of message I don't handle for file saving.").await?;
            } else {
                
                log::debug!("Ignoring unhandled message type: {:?}", message.kind);
            }
        }
    }
    Ok(()) 
}


fn get_file_info_from_message(message: &Message) -> Option<(FileMeta, String)> {
    if let MessageKind::Common(common) = &message.kind {
        match &common.media_kind {
            MediaKind::Document(MediaDocument { document, .. }) => {
                log::info!("Received document: {:?}", document.file_name);
                Some((
                    document.file.clone(),
                    document
                        .file_name
                        .clone()
                        .unwrap_or_else(|| "unknown_document".to_string()),
                ))
            }
            MediaKind::Photo(MediaPhoto { photo, .. }) => {
                if let Some(photo_size) = photo.last() {
                    let file_name = format!("{}.jpg", photo_size.file.unique_id);
                    log::info!("Received photo: {}", file_name);
                    Some((photo_size.file.clone(), file_name))
                } else {
                    log::warn!("Received a photo message with no photo sizes.");
                    None
                }
            }
            MediaKind::Video(MediaVideo { video, .. }) => {
                log::info!("Received video: {:?}", video.file_name);
                Some((
                    video.file.clone(),
                    video
                        .file_name
                        .clone()
                        .unwrap_or_else(|| "unknown_video.mp4".to_string()),
                ))
            }
            MediaKind::Audio(MediaAudio { audio, .. }) => {
                log::info!("Received audio: {:?}", audio.file_name);
                Some((
                    audio.file.clone(),
                    audio
                        .file_name
                        .clone()
                        .unwrap_or_else(|| "unknown_audio.mp3".to_string()),
                ))
            }
            MediaKind::Voice(MediaVoice { voice, .. }) => {
                let file_name = format!("{}.ogg", voice.file.unique_id); 
                log::info!("Received voice message: {}", file_name);
                Some((voice.file.clone(), file_name))
            }
            _ => {
                log::info!("Received a common message with media kind that is not a directly downloadable file I'm configured for.");
                None
            }
        }
    } else {
        log::info!("Received a message that is not a 'Common' message kind (e.g., new chat member): {:?}", message.kind);
        None
    }
}


fn sanitize_filename(filename: String) -> String {
    let sanitized = filename.replace(|c: char| !(c.is_alphanumeric() || c == '.' || c == '_' || c == '-'), "_");
    if sanitized.is_empty() || sanitized == "." || sanitized == ".." {
        "sanitized_file".to_string() 
    } else {
        sanitized
    }
}
