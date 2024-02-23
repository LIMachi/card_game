use crate::cards::assets::serializer::AssetLoadderError;
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAssetUsages;
use meshtext::{MeshGenerator, MeshText, OwnedFace, TextSection};
use regex::Regex;

#[derive(Default)]
pub struct Font3DLoader;

#[derive(Asset, TypePath)]
pub struct Font3D(MeshGenerator<OwnedFace>);

#[derive(Component)]
pub struct RecomputeGlyphs;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Text3D {
    pub text: String,
    pub flat: bool,
}

impl From<&str> for Text3D {
    fn from(value: &str) -> Self {
        Self {
            text: value.to_string(),
            flat: false,
        }
    }
}

impl From<String> for Text3D {
    fn from(value: String) -> Self {
        Self {
            text: value.clone(),
            flat: false,
        }
    }
}

impl Font3D {
    pub fn build(&mut self, text: &String, flat: bool, mut transform: Transform) -> Mesh {
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut out = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        let down = transform.down();
        let new_line_splitter = Regex::new("\n\r|\r\n|\r|\n").unwrap();
        for sub in new_line_splitter.split(text) {
            let tm: MeshText = self
                .0
                .generate_section(sub, flat, Some(transform.compute_matrix().as_ref()))
                .unwrap();
            for chunk in tm.vertices.chunks(3) {
                positions.push([chunk[0], chunk[1], chunk[2]]);
            }
            transform.translation += *down;
        }
        let len = positions.len();
        out.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        out.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0f32, 0f32]; len]);
        out.compute_flat_normals();
        out
    }

    pub fn build_from_text3d(&mut self, text: &Text3D) -> Mesh {
        self.build(&text.text, text.flat, Transform::IDENTITY)
    }
}

impl AssetLoader for Font3DLoader {
    type Asset = Font3D;
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
            Ok(Font3D(MeshGenerator::new(bytes)))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ttf"]
    }
}

#[derive(Bundle, Default)]
pub struct Text3DBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub font: Handle<Font3D>,
    pub text: Text3D,
}

impl Text3DBundle {
    pub fn new<S: Into<Text3D>>(text: S) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn with_font(mut self, font: Handle<Font3D>) -> Self {
        self.font = font;
        self
    }

    pub fn with_material(mut self, material: Handle<StandardMaterial>) -> Self {
        self.material = material;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.transform.rotation = rotation;
        self
    }

    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.transform.translation = translation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.transform.scale = scale;
        self
    }

    pub fn with_flat(mut self, flat: bool) -> Self {
        self.text.flat = flat;
        self
    }
}

pub fn test(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_loader: Res<AssetServer>,
) {
    commands.spawn(
        Text3DBundle::new("test\nwith new line")
            .with_rotation(Quat::from_rotation_x(90f32.to_radians()))
            .with_font(asset_loader.load::<Font3D>("fonts/FiraMono-Medium.ttf"))
            .with_material(materials.add(Color::rgb(1f32, 0f32, 0f32))),
    );
}

pub fn queue_glyph_update(
    mut commands: Commands,
    update: Query<
        Entity,
        (
            Without<RecomputeGlyphs>,
            Or<(Changed<Text3D>, Changed<Handle<Font3D>>)>,
        ),
    >,
) {
    for entity in update.iter() {
        commands.entity(entity).insert(RecomputeGlyphs);
    }
}

pub fn update_glyphs(
    mut commands: Commands,
    mut update: Query<(Entity, &Handle<Font3D>, &mut Handle<Mesh>, &Text3D), With<RecomputeGlyphs>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fonts: ResMut<Assets<Font3D>>,
) {
    for (entity, font, mut mesh, text) in update.iter_mut() {
        if let Some(font) = fonts.get_mut(font) {
            *mesh = meshes.add(font.build_from_text3d(text));
            commands.entity(entity).remove::<(RecomputeGlyphs, Aabb)>(); //Remove Aabb to force the recompute of it by 'calculate_bounds' (bevy_render::visibility::mod 260) in post update
        }
    }
}

///minimalist plugin to load and use ttf as a dynamic mesh builder
pub struct Font3DPlugin;

impl Plugin for Font3DPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Text3D>()
            .init_asset::<Font3D>()
            .init_asset_loader::<Font3DLoader>()
            .add_systems(Update, queue_glyph_update)
            .add_systems(Update, update_glyphs.after(queue_glyph_update));
    }
}
