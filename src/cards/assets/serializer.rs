use crate::cards::assets::{Card, Deck};
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::*;
use thiserror::Error;

#[derive(Default)]
pub struct CardLoadder;

#[derive(Default)]
pub struct DeckLoadder;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AssetLoadderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for CardLoadder {
    type Asset = Card;
    type Settings = ();
    type Error = AssetLoadderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            Ok(ron::de::from_bytes::<Card>(&bytes)?)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["card.ron"]
    }
}

impl AssetLoader for DeckLoadder {
    type Asset = Deck;
    type Settings = ();
    type Error = AssetLoadderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            Ok(Deck(ron::de::from_bytes::<Vec<(u32, String)>>(&bytes)?))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["deck.ron"]
    }
}
