export interface Unit {
  unit_id?: string;
  singular_name: string;
  plural_name: string;
}

export interface Ingredient {
  ingredient_id?: string;
  singular_name: string;
  plural_name: string;
}

export interface CompactRecipeIngredient {
  ingredient_id?: number;
  unit_id?: number;
  quantity: string;
}
export interface DetailedRecipeIngredient{
  ingredient: Ingredient,
  unit: Unit,
  quantity: string
}
export interface RecipeStep {
  // step_id?: number;
  step_number: number,
  instruction: string
}
export interface DetailedRecipe {
  recipe_id: number,
  name: string,
  description: string,
  recipe_ingredients: DetailedRecipeIngredient[]
}
export interface Recipe {
  recipe_id?: number;
  name: string;
  description: string;
  // ingredients: RecipeIngredient[];
  // steps: RecipeStep[];
}
export interface GetRecipesResponse {
  recipes: Recipe[];
  next_start_from?: number;
  previous_start_from?: number;
}