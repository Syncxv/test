pub enum PatchType {
    Friend,
    Library,
}

impl PatchType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "friend" => Some(Self::Friend),
            "library" => Some(Self::Library),
            _ => None,
        }
    }
}

#[tauri::command]
pub fn patch(patch_type: String) -> bool {
    let patch_type = match PatchType::from_str(&patch_type) {
        Some(pt) => pt,
        None => {
            eprintln!("Invalid patch type: {}", patch_type);
            return false;
        }
    };

    match patch_type {
        PatchType::Friend => {
            println!("Patching friend...");
        }
        PatchType::Library => {
            println!("Patching library...");
        }
    }

    true
}
