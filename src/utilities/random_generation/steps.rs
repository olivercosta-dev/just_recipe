
use fake::{Fake, Faker};

use crate::recipe::recipe_step::RecipeStep;

// Generates a random number of `RecipeStep` instances.
///
/// This function generates a random number of `RecipeStep` instances, with the number of steps
/// ranging from 2 to 10. Each step is assigned a step number and a randomly generated instruction.
///
/// # Returns
/// - `Vec<RecipeStep>`: A vector containing the generated `RecipeStep` instances.
pub fn generate_random_number_of_steps() -> Vec<RecipeStep> {
    let number_of_steps = (2..=10).fake::<i32>();
    (1..number_of_steps)
        .map(|step_number| RecipeStep {
            step_id: 0,
            recipe_id: 0,
            step_number,
            instruction: Faker.fake::<String>(),
        })
        .collect()
}