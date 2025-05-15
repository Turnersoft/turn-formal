use turn_formal::{subjects::math::formalism::registry::get_theorems, turn_render::ToTurnMath};

fn main() {
    let theorems = get_theorems();

    for (i, theorem) in theorems.iter().enumerate() {
        let math_node = theorem.to_turn_math(theorem.id.clone());
        println!("Theorem {}: {:#?}", i, serde_json::to_string(&math_node));
    }
}
