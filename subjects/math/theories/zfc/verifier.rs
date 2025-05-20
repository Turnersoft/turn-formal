use super::axioms::{SatisfiesZFC, ZFCAxioms};
use super::is_subset_of;
use super::set::{Set, SetElement};

/// ZFC Axiom Verification Implementation
pub struct ZFCVerifier;

impl ZFCAxioms for ZFCVerifier {
    type Set = Set;

    fn verify_extensionality(set1: &Self::Set, set2: &Self::Set) -> bool {
        set1 == set2
    }

    fn verify_empty_set(set: &Self::Set) -> bool {
        set.is_empty()
    }

    fn verify_pairing(pair: &Self::Set, a: &Self::Set, b: &Self::Set) -> bool {
        let a_element = SetElement::from(a.clone());
        let b_element = SetElement::from(b.clone());

        if a == b {
            pair.len() == 1 && pair.contains(&a_element)
        } else {
            pair.len() == 2 && pair.contains(&a_element) && pair.contains(&b_element)
        }
    }

    fn verify_union(union: &Self::Set, sets: &[Self::Set]) -> bool {
        // Transform sets into SetElements for checking containment
        let set_elements: Vec<SetElement> = sets
            .iter()
            .map(|set| SetElement::from(set.clone()))
            .collect();

        // Check that each element from each set is in the union
        sets.iter().all(|set| {
            set.elements().iter().all(|elem| union.contains(elem))
        }) &&
        // Check that each element in the union is from one of the sets
        union.elements().iter().all(|elem| {
            sets.iter().any(|set| set.contains(elem))
        })
    }

    fn verify_power_set(power: &Self::Set, original: &Self::Set) -> bool {
        // Check that each element in the power set is a subset of the original
        power.elements().iter().all(|subset_elem| {
            match subset_elem {
                SetElement::Set(subset) => {
                    subset.elements().iter().all(|elem| original.contains(elem))
                }
                _ => false, // Only Set elements should be in a power set
            }
        })
    }

    fn verify_foundation(set: &Self::Set) -> bool {
        set.is_well_founded()
    }

    fn verify_separation<P>(subset: &Self::Set, superset: &Self::Set, property: P) -> bool
    where
        P: Fn(&Self::Set) -> bool,
    {
        // Check subset relation
        is_subset_of(subset, superset) &&
        // Check that every element in subset satisfies the property
        subset.elements().iter().all(|e| {
            match e {
                SetElement::Set(set) => property(set),
                _ => false // For non-set elements, property can't be verified
            }
        }) &&
        // Check that every element in superset that satisfies property is in subset
        superset.elements().iter().filter(|e| {
            match e {
                SetElement::Set(set) => property(set),
                _ => false // For non-set elements, property can't be verified
            }
        }).all(|e| subset.contains(e))
    }

    fn verify_replacement<F>(image: &Self::Set, domain: &Self::Set, function: F) -> bool
    where
        F: Fn(&Self::Set) -> Self::Set,
    {
        // Check that every element in domain maps to something in image
        domain.elements().iter().all(|elem| {
            match elem {
                SetElement::Set(set) => {
                    let fn_result = function(set);
                    let fn_elem = SetElement::from(fn_result);
                    image.contains(&fn_elem)
                },
                _ => false // Only Set elements can be mapped by F
            }
        }) &&
        // Check that every element in image comes from domain
        image.elements().iter().all(|y| {
            domain.elements().iter().any(|x| {
                match x {
                    SetElement::Set(set) => {
                        let fn_result = function(set);
                        let fn_elem = SetElement::from(fn_result);
                        &fn_elem == y
                    },
                    _ => false // Only Set elements can be mapped by F
                }
            })
        })
    }

    fn verify_choice(choice: &Self::Set, sets: &[Self::Set]) -> bool {
        println!("[Verify Choice] Starting verification");
        println!(
            "[Verify Choice] Choice set elements: {:?}",
            choice.elements()
        );

        sets.iter().filter(|set| !set.is_empty()).all(|set| {
            println!("\n[Verify Choice] Checking set: {:?}", set);
            println!("[Verify Choice] Set elements: {:?}", set.elements());

            let mut count = 0;
            for elem in set.elements() {
                print!("[Verify Choice] Checking element {:?}... ", elem);
                if choice.contains(&elem) {
                    count += 1;
                    println!("FOUND (count={})", count);
                    if count > 1 {
                        println!("[Verify Choice] ❌ Multiple elements found in set!");
                        return false;
                    }
                } else {
                    println!("NOT found");
                }
            }
            println!("[Verify Choice] Final count for set: {}", count);
            count == 1
        })
    }
}

impl SatisfiesZFC for ZFCVerifier {}
