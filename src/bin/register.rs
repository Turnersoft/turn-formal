use turn_formal::subjects::math::formalism::registry::get_theorems;

fn main() {
    let theorems = get_theorems();
    println!("{:?}", theorems);
}
