pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

use Allergen::*;

const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            allergens: Self::calculate_allergens(score),
        }
    }

    fn calculate_allergens(score: u32) -> Vec<Allergen> {
        let mut allergens = Vec::new();

        for flag in 0..=7 {
            if score & 1 << flag != 0 {
                allergens.push(ALLERGENS[flag as usize]);
            }
        }

        allergens
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.to_owned()
    }
}
