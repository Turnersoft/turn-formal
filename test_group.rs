use turn_formal::subjects::math::theories::groups::definitions::{
    FinitePropertyVariant, GroupProperty,
};
use turn_formal::subjects::math::theories::groups::helpers::{cyclic_group, symmetric_group};

fn main() {
    let z5 = cyclic_group(5);
    let s3 = symmetric_group(3);

    println!("Cyclic group Z_5 properties:");
    for p in &z5.properties {
        match p {
            GroupProperty::Finite(FinitePropertyVariant::Finite(n)) => {
                println!("  Finite property value: {}", n);
            }
            _ => println!("  Other property: {:?}", p),
        }
    }

    println!("\nSymmetric group S_3 properties:");
    for p in &s3.properties {
        match p {
            GroupProperty::Finite(FinitePropertyVariant::Finite(n)) => {
                println!("  Finite property value: {}", n);
            }
            _ => println!("  Other property: {:?}", p),
        }
    }
}
