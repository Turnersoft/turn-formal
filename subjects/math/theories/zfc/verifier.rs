use super::axioms::{SatisfiesZFC, ZFCAxioms};
use super::Set;

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
        if a == b {
            pair.len() == 1 && pair.contains(a)
        } else {
            pair.len() == 2 && pair.contains(a) && pair.contains(b)
        }
    }

    fn verify_union(union: &Self::Set, sets: &[Self::Set]) -> bool {
        sets.iter()
            .all(|set| set.elements().iter().all(|elem| union.contains(elem)))
            && union
                .elements()
                .iter()
                .all(|elem| sets.iter().any(|set| set.contains(elem)))
    }

    fn verify_power_set(power: &Self::Set, original: &Self::Set) -> bool {
        power
            .elements()
            .iter()
            .all(|subset| subset.elements().iter().all(|elem| original.contains(elem)))
    }

    fn verify_foundation(set: &Self::Set) -> bool {
        set.is_well_founded()
    }

    fn verify_separation<P>(subset: &Self::Set, superset: &Self::Set, property: P) -> bool
    where
        P: Fn(&Self::Set) -> bool,
    {
        subset.is_subset_of(superset)
            && subset.elements().iter().all(|e| property(e))
            && superset
                .elements()
                .iter()
                .filter(|e| property(e))
                .all(|e| subset.contains(e))
    }

    fn verify_replacement<F>(image: &Self::Set, domain: &Self::Set, function: F) -> bool
    where
        F: Fn(&Self::Set) -> Self::Set,
    {
        domain
            .elements()
            .iter()
            .all(|elem| image.contains(&function(elem)))
            && image
                .elements()
                .iter()
                .all(|y| domain.elements().iter().any(|x| function(x) == *y))
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
