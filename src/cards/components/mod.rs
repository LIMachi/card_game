use bevy::prelude::*;

pub mod factions;
pub mod kinds;
pub mod owners;

pub mod prelude {
    pub use super::{
        factions::{Blob, CardFactions, MachineCult, Neutral, StarEmpire, TradeFederation},
        kinds::{Base, CardKinds, Outpost, Ship},
        owners::{CardOwners, MarketOnwed},
        CardCost, CardIndex, CardVisibility,
    };
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct CardIndex(pub usize);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct CardCost(pub i32);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum CardVisibility {
    #[default]
    Hidden,
    Visible,
}

pub struct CardComponentsPlugin;

impl Plugin for CardComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CardIndex>()
            .register_type::<CardCost>()
            .register_type::<CardVisibility>()
            .add_plugins((
                factions::FactionsPlugin,
                kinds::KindsPlugin,
                owners::OwnersPlugin,
            ));
    }
}
