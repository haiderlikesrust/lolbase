use crate::error::ApiError;


#[derive(Debug, Clone)]
pub enum MediaType {
    VideoMp4,
    ImagePng,
    ImageJpeg,
    AudioOgg,
    VideoOgg,
    VideoMpeg,
    VideoWebm,
    ImageWebp,
}

impl TryFrom<&str> for MediaType {
    type Error = ApiError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "video/ogg" => Ok(Self::VideoOgg),
            "audio/ogg" => Ok(Self::AudioOgg),
            "image/png" => Ok(Self::ImagePng),
            "image/jpeg" => Ok(Self::ImageJpeg),
            "image/webp" => Ok(Self::ImageWebp),
            "video/mp4" => Ok(Self::VideoMp4),
            "video/webm" => Ok(Self::VideoWebm),
            _ => Err(ApiError::UnsupportedMedia),
        }
    }
}
