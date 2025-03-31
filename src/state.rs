use serde_json::{json, Value, Map};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

/// Lee el archivo JSON y devuelve un `Map<String, Value>`. Si el archivo no existe o está vacío, devuelve `{}`.
pub fn read_file(file_name: &str) -> Map<String, Value> {
    // Intenta abrir el archivo
    let mut file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => return Map::new(), // Si no existe, devuelve un JSON vacío
    };

    let mut data = String::new();
    if file.read_to_string(&mut data).is_err() || data.trim().is_empty() {
        return Map::new(); // Si hay un error al leer o el archivo está vacío, devuelve `{}`.
    }

    // Intenta parsear el JSON
    match serde_json::from_str::<Value>(&data) {
        Ok(Value::Object(map)) => map, // Si el JSON es un objeto, lo devuelve
        _ => Map::new(), // Si no es un objeto válido, devuelve `{}`.
    }
}

/// Escribe el estado actualizado en el archivo JSON
pub fn write_to_file(file_name: &str, state: &Map<String, Value>) {
    let new_data = json!(state); // Convierte `state` a JSON
    let mut file = File::create(file_name).expect("No se pudo abrir el archivo"); // Crea/abre el archivo
    file.write_all(new_data.to_string().as_bytes()).expect("Error escribiendo en el archivo"); // Escribe JSON en el archivo
}
