use std::path::Path;

use tokio::fs;
use tokio::io::AsyncWriteExt;

use grammers_tl_types as tl;

use crate::ClientHandle;

pub enum PhotoSize {
    Empty(SizeEmpty),
    Size(Size),
    Cached(CachedSize),
    Stripped(StrippedSize),
    Progressive(ProgressiveSize),
    Path(PathSize),
}

impl PhotoSize {
    pub fn make_from(
        size: &tl::enums::PhotoSize,
        photo: &tl::types::Photo,
        client: ClientHandle,
    ) -> Self {
        match size {
            tl::enums::PhotoSize::Empty(size) => PhotoSize::Empty(SizeEmpty {
                photo_type: size.r#type.clone(),
            }),
            tl::enums::PhotoSize::Size(size) => PhotoSize::Size(Size {
                photo_type: size.r#type.clone(),
                width: size.w,
                height: size.h,
                size: size.size,
                id: photo.id,
                access_hash: photo.access_hash,
                file_reference: photo.file_reference.clone(),
                client,
            }),
            tl::enums::PhotoSize::PhotoCachedSize(size) => PhotoSize::Cached(CachedSize {
                photo_type: size.r#type.clone(),
                width: size.w,
                height: size.h,
                bytes: size.bytes.clone(),
            }),
            tl::enums::PhotoSize::PhotoStrippedSize(size) => PhotoSize::Stripped(StrippedSize {
                photo_type: size.r#type.clone(),
                bytes: size.bytes.clone(),
            }),
            tl::enums::PhotoSize::Progressive(size) => PhotoSize::Progressive(ProgressiveSize {
                photo_type: size.r#type.clone(),
                width: size.w,
                height: size.h,
                sizes: size.sizes.clone(),
            }),
            tl::enums::PhotoSize::PhotoPathSize(size) => PhotoSize::Path(PathSize {
                photo_type: size.r#type.clone(),
                bytes: size.bytes.clone(),
            }),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            PhotoSize::Empty(_) => 0,
            PhotoSize::Size(size) => size.size as usize,
            PhotoSize::Cached(size) => size.bytes.len(),
            PhotoSize::Stripped(size) => size.bytes.len(),
            PhotoSize::Progressive(size) => size.sizes.iter().sum::<i32>() as usize,
            PhotoSize::Path(size) => size.bytes.len(),
        }
    }

    pub async fn download(&self, path: &Path) {
        match self {
            PhotoSize::Empty(_) => {
                fs::File::create(path).await.unwrap();
            }
            PhotoSize::Size(size) => {
                let input_location = tl::types::InputPhotoFileLocation {
                    id: size.id.clone(),
                    access_hash: size.access_hash.clone(),
                    file_reference: size.file_reference.clone(),
                    thumb_size: size.photo_type.clone(),
                };
                size.client
                    .clone()
                    .download_media_at_location(input_location, path)
                    .await
                    .unwrap();
            }
            PhotoSize::Cached(size) => {
                let mut file = fs::File::create(path).await.unwrap();
                file.write(&size.bytes).await.unwrap();
            }
            PhotoSize::Stripped(_) => {
                todo!("Not yet implemented")
            }
            PhotoSize::Progressive(_) => {
                todo!("Not yet implemented")
            }
            PhotoSize::Path(_) => {
                todo!("Not yet implemented")
            }
        };
    }

    pub fn photo_type(&self) -> String {
        match self {
            PhotoSize::Empty(size) => size.photo_type.clone(),
            PhotoSize::Size(size) => size.photo_type.clone(),
            PhotoSize::Cached(size) => size.photo_type.clone(),
            PhotoSize::Stripped(size) => size.photo_type.clone(),
            PhotoSize::Progressive(size) => size.photo_type.clone(),
            PhotoSize::Path(size) => size.photo_type.clone(),
        }
    }
}

pub struct SizeEmpty {
    photo_type: String,
}

pub struct Size {
    photo_type: String,
    pub width: i32,
    pub height: i32,
    pub size: i32,

    id: i64,
    access_hash: i64,
    file_reference: Vec<u8>,

    client: ClientHandle,
}

pub struct CachedSize {
    photo_type: String,

    pub width: i32,
    pub height: i32,
    pub bytes: Vec<u8>,
}

pub struct StrippedSize {
    photo_type: String,

    pub bytes: Vec<u8>,
}

pub struct ProgressiveSize {
    photo_type: String,

    pub width: i32,
    pub height: i32,
    pub sizes: Vec<i32>,
}

pub struct PathSize {
    photo_type: String,

    pub bytes: Vec<u8>,
}

pub trait VecExt {
    fn largest(&self) -> Option<&PhotoSize>;
}

impl VecExt for Vec<PhotoSize> {
    fn largest(&self) -> Option<&PhotoSize> {
        self.iter().max_by_key(|x| x.size())
    }
}
