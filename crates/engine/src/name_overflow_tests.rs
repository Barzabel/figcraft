//! Names that end in a number at the top of `u64` (#309, finding 3): duplicating a layer with
//! renamed duplicates on, and creating a frame next to one, never overflow the number.

use serde_json::{Value, json};

use crate::Session;
use crate::params::id_of;

fn ex(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd}: {e}"))
}

fn name_of(s: &Session, v: &Value) -> String {
    let id = id_of(v).unwrap();
    s.doc().unwrap().node(id).unwrap().name.clone()
}

#[test]
fn a_duplicate_of_a_layer_numbered_at_the_top_gets_another_number() {
    let mut s = Session::new();
    ex(&mut s, "pref.renameDuplicates", json!({"on": true}));
    let r = ex(&mut s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 10, "height": 10}))["id"].clone();
    let top = format!("Layer {}", u64::MAX);
    ex(&mut s, "object.rename", json!({"ids": [r], "name": top}));
    let out = ex(&mut s, "edit.duplicate", json!({"ids": [r]}));
    assert_eq!(name_of(&s, &out["ids"][0]), format!("{top} 2"));
    // An ordinary number still counts up.
    ex(&mut s, "object.rename", json!({"ids": [r], "name": "Layer 4"}));
    let out = ex(&mut s, "edit.duplicate", json!({"ids": [r]}));
    assert_eq!(name_of(&s, &out["ids"][0]), "Layer 5");
}

#[test]
fn a_new_frame_next_to_one_numbered_at_the_top_is_named() {
    let mut s = Session::new();
    let f = ex(&mut s, "frame.create", json!({"x": 0, "y": 0, "width": 10, "height": 10}))["id"].clone();
    ex(&mut s, "object.rename", json!({"ids": [f], "name": format!("Frame {}", u64::MAX)}));
    let g = ex(&mut s, "frame.create", json!({"x": 20, "y": 0, "width": 10, "height": 10}));
    assert_eq!(name_of(&s, &g["id"]), "Frame 1");
}
