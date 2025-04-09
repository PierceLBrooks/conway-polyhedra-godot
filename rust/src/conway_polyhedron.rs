use godot::classes::{Engine, FileAccess, Mesh, MeshInstance3D, IMeshInstance3D, ProjectSettings, ResourceLoader, resource_loader::CacheMode};
use godot::global::str;
use godot::prelude::*;
use polyhedron_ops::*;
use std::path::*;

#[derive(GodotClass)]
#[class(base=MeshInstance3D, init, tool)]
pub struct ConwayPolyhedron {
    #[init(val = false)]
    pub edit: bool,
    #[init(val = GString::from(""))]
    pub old: GString,
    #[export]
    #[init(val = GString::from(""))]
    pub recipe: GString,

    base: Base<MeshInstance3D>,
}

#[godot_api]
impl ConwayPolyhedron {
    #[func]
    fn export_as_obj(&mut self, path: GString) -> bool {
        let full = str(&[path.to_variant(), self.recipe.to_variant(), GString::from(".obj").to_variant()]);
        if FileAccess::file_exists(&full) {
            let obj = match ResourceLoader::singleton().load_ex(&full).cache_mode(CacheMode::REPLACE_DEEP).done() {
                Some(o) => o,
                None => {
                    godot_error!("{}", full);
                    return false;
                }
            };
            self.base().get_node_as::<MeshInstance3D>(".").set_mesh(&obj.cast::<Mesh>());
            return true;
        }

        unsafe {
            let poly = match Polyhedron::try_from(String::from_utf8_unchecked(self.recipe.to_utf8_buffer().to_vec()).as_str()) {
                Ok(p) => p,
                Err(e) => {
                    godot_error!("{}", e);
                    return false;
                }
            };
            let buf = String::from_utf8_unchecked(path.to_utf8_buffer().to_vec());
            let fd = Path::new(buf.as_str());
            let res = match poly.write_obj(fd, true) {
                Ok(r) => r,
                Err(e) => {
                    godot_error!("{} @ {}", e, fd.display());
                    return false;
                }
            };
            let lossy = String::from(res.to_string_lossy());
            let temp = GString::from(lossy);
            let obj = match ResourceLoader::singleton().load_ex(&temp).cache_mode(CacheMode::REPLACE_DEEP).done() {
                Some(o) => o,
                None => {
                    godot_error!("{}", res.display());
                    return false;
                }
            };
            self.base().get_node_as::<MeshInstance3D>(".").set_mesh(&obj.cast::<Mesh>());
        }

        return true;
    }
}

#[godot_api]
impl IMeshInstance3D for ConwayPolyhedron {
    fn set_property(&mut self, _property: StringName, _value: Variant) -> bool {
        self.edit = true;
        return false;
    }

    fn process(&mut self, _delta: f32) {
        if !Engine::singleton().is_editor_hint() || !self.edit || self.recipe.is_empty() {
            return;
        }
        if self.old == self.recipe && (self.base().get_node_as::<MeshInstance3D>(".").get_mesh().is_none() || self.base().get_node_as::<MeshInstance3D>(".").get_mesh().unwrap().get_path().contains(&self.recipe)) {
            return;
        }
        let path = GString::from("res://");
        if !self.export_as_obj(ProjectSettings::singleton().globalize_path(&path)) {
            self.base().get_node_as::<MeshInstance3D>(".").set_mesh(&Mesh::new_gd());
        }
        self.edit = false;
    }
}
