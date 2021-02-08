// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use crate::types::photo_sizes::PhotoSize;
use crate::ClientHandle;
use grammers_tl_types as tl;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::path::Path;

#[derive(Clone)]
pub struct Photo {
    photo: tl::types::MessageMediaPhoto,
    client: ClientHandle,
}

#[derive(Clone)]
pub struct Document {
    document: tl::types::MessageMediaDocument,
    client: ClientHandle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Uploaded {
    pub(crate) input_file: tl::enums::InputFile,
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Media {
    Photo(Photo),
    Document(Document),
    Uploaded(Uploaded),
}

impl Document {
    pub(crate) fn from_media(
        document: tl::types::MessageMediaDocument,
        client: ClientHandle,
    ) -> Self {
        Self { document, client }
    }

    pub async fn download(&mut self, path: &Path) {
        match &self.document.document {
            Some(_) => {
                if let Some(location) = self.to_input_location() {
                    self.client
                        .download_media_at_location(location, path)
                        .await
                        .unwrap();
                }
            }
            None => (),
        }
    }

    fn to_input_location(&self) -> Option<tl::enums::InputFileLocation> {
        self.document.document.as_ref().and_then(|p| match p {
            tl::enums::Document::Empty(_) => None,
            tl::enums::Document::Document(doc) => Some(
                tl::types::InputDocumentFileLocation {
                    id: doc.id,
                    access_hash: doc.access_hash,
                    file_reference: doc.file_reference.clone(),
                    thumb_size: String::new(),
                }
                .into(),
            ),
        })
    }
}

impl Photo {
    pub(crate) fn from_raw(photo: tl::enums::Photo, client: ClientHandle) -> Self {
        Self {
            photo: tl::types::MessageMediaPhoto {
                photo: Some(photo),
                ttl_seconds: None,
            },
            client,
        }
    }

    pub(crate) fn from_media(photo: tl::types::MessageMediaPhoto, client: ClientHandle) -> Self {
        Self { photo, client }
    }

    fn to_input_location(&self) -> Option<tl::enums::InputFileLocation> {
        use tl::enums::Photo as P;

        self.photo.photo.as_ref().and_then(|p| match p {
            P::Empty(_) => None,
            P::Photo(photo) => Some(
                tl::types::InputPhotoFileLocation {
                    id: photo.id,
                    access_hash: photo.access_hash,
                    file_reference: photo.file_reference.clone(),
                    thumb_size: String::new(),
                }
                .into(),
            ),
        })
    }

    pub fn id(&self) -> i64 {
        use tl::enums::Photo as P;

        match self.photo.photo.as_ref().unwrap() {
            P::Empty(photo) => photo.id,
            P::Photo(photo) => photo.id,
        }
    }

    /// Get photo thumbs.
    pub fn thumbs(&self) -> Vec<PhotoSize> {
        use tl::enums::Photo as P;

        let photo = match self.photo.photo.as_ref() {
            Some(photo) => photo,
            None => return vec![],
        };

        match photo {
            P::Empty(_) => vec![],
            P::Photo(photo) => photo
                .sizes
                .iter()
                .map(|x| PhotoSize::make_from(&x, &photo, self.client.clone()))
                .collect(),
        }
    }
}

impl Debug for Photo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.photo)
    }
}

impl PartialEq for Photo {
    fn eq(&self, other: &Self) -> bool {
        self.photo == other.photo
    }
}

impl Debug for Document {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.document)
    }
}

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.document == other.document
    }
}

impl Uploaded {
    pub(crate) fn from_raw(input_file: tl::enums::InputFile) -> Self {
        Self { input_file }
    }

    pub(crate) fn name(&self) -> &str {
        match &self.input_file {
            tl::enums::InputFile::File(f) => f.name.as_ref(),
            tl::enums::InputFile::Big(f) => f.name.as_ref(),
        }
    }
}

impl Media {
    pub(crate) fn from_raw(media: tl::enums::MessageMedia, client: ClientHandle) -> Option<Self> {
        use tl::enums::MessageMedia as M;

        // TODO implement the rest
        match media {
            M::Empty => None,
            M::Photo(photo) => Some(Self::Photo(Photo::from_media(photo, client))),
            M::Geo(_) => None,
            M::Contact(_) => None,
            M::Unsupported => None,
            M::Document(document) => Some(Self::Document(Document::from_media(document, client))),
            M::WebPage(_) => None,
            M::Venue(_) => None,
            M::Game(_) => None,
            M::Invoice(_) => None,
            M::GeoLive(_) => None,
            M::Poll(_) => None,
            M::Dice(_) => None,
        }
    }

    pub(crate) fn to_input_location(&self) -> Option<tl::enums::InputFileLocation> {
        match self {
            Media::Photo(photo) => photo.to_input_location(),
            Media::Uploaded(_) => None,
            Media::Document(doc) => doc.to_input_location(),
        }
    }
}

impl From<Photo> for Media {
    fn from(photo: Photo) -> Self {
        Self::Photo(photo)
    }
}

impl From<Uploaded> for Media {
    fn from(uploaded: Uploaded) -> Self {
        Self::Uploaded(uploaded)
    }
}
