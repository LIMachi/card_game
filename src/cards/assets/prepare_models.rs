use crate::cards::assets::{LoadedModels, RawModel};
use bevy::asset::LoadedFolder;
use bevy::gltf::{Gltf, GltfMesh};
use bevy::prelude::*;

pub fn prepare_models(
    raw_model: Res<RawModel>,
    mut loaded_models: ResMut<LoadedModels>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    folders: Res<Assets<LoadedFolder>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(gltf) = gltfs.get(&raw_model.gltf) {
        if let Some(ships) = folders.get(&raw_model.ships).map(|f| f.handles.clone()) {
            if let Some(bases) = folders.get(&raw_model.bases).map(|f| f.handles.clone()) {
                let front_ship = &gltf_meshes
                    .get(&gltf.named_meshes["Ship"])
                    .unwrap()
                    .primitives[0];
                let front_base = &gltf_meshes
                    .get(&gltf.named_meshes["Base"])
                    .unwrap()
                    .primitives[0];

                let back = &gltf_meshes
                    .get(&gltf.named_meshes["Back"])
                    .unwrap()
                    .primitives[0];
                loaded_models.back_mesh = back.mesh.clone();
                loaded_models.back_material = back.material.as_ref().unwrap().clone();
                let side = &gltf_meshes
                    .get(&gltf.named_meshes["Side"])
                    .unwrap()
                    .primitives[0];
                loaded_models.side_mesh = side.mesh.clone();
                loaded_models.side_material = side.material.as_ref().unwrap().clone();
                loaded_models.front_ship_mesh = front_ship.mesh.clone();
                loaded_models.front_base_mesh = front_base.mesh.clone();
                let mat_ship = materials
                    .get(front_ship.material.clone().unwrap())
                    .unwrap()
                    .clone();
                let mat_base = materials
                    .get(front_base.material.clone().unwrap())
                    .unwrap()
                    .clone();
                for h in &ships {
                    loaded_models.front_materials.insert(
                        h.path()
                            .unwrap()
                            .path()
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                        materials.add(StandardMaterial {
                            base_color_texture: Some(h.clone().typed::<Image>()),
                            ..mat_ship.clone()
                        }),
                    );
                }
                for h in &bases {
                    loaded_models.front_materials.insert(
                        h.path()
                            .unwrap()
                            .path()
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                        materials.add(StandardMaterial {
                            base_color_texture: Some(h.clone().typed::<Image>()),
                            ..mat_base.clone()
                        }),
                    );
                }
                loaded_models.ready = true;
            }
        }
    }
}
