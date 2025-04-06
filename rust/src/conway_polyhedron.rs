use godot::classes::{Engine, FileAccess, MeshInstance3D, IMeshInstance3D, ProjectSettings};
use godot::global::str;
use godot::prelude::*;
use polyhedron_ops::*;
use std::path::*;

#[derive(GodotClass)]
#[class(base=MeshInstance3D, init, tool)]
pub struct ConwayPolyhedron {
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
    fn export_as_obj(&mut self, path: GString) {
        if FileAccess::file_exists(&path) {
            return;
        }

        unsafe {
            let poly = match Polyhedron::try_from(String::from_utf8_unchecked(self.recipe.to_utf8_buffer().to_vec()).as_str()) {
                Ok(p) => p,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            println!("{}", poly.write_obj(Path::new(String::from_utf8_unchecked(path.to_utf8_buffer().to_vec()).as_str()), true).unwrap().display());
        }
    }
}

#[godot_api]
impl IMeshInstance3D for ConwayPolyhedron {
    fn process(&mut self, _delta: f32) {
        if !Engine::singleton().is_editor_hint() {
            return;
        }
        if self.old == self.recipe {
            return;
        }
        if self.recipe.is_empty() {
            return;
        }
        let path = str(&[GString::from("res://").to_variant(), self.recipe.to_variant(), GString::from(".obj").to_variant()]);
        self.export_as_obj(ProjectSettings::singleton().globalize_path(&path));
        //self.export_as_obj(ProjectSettings::singleton().globalize_path(GString::from(str(&[GString::from("res://").to_variant(), self.recipe.to_variant(), GString::from(".obj").to_variant()])).arg()));
        //self.export_as_obj(ProjectSettings::singleton().globalize_path(&str(&[GString::from("res://").to_variant(), self.recipe.to_variant(), GString::from(".obj").to_variant()]).arg()));
        self.old = self.recipe.clone();
        //self.base_mut().set_mesh();
    }
}
