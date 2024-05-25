import { createSignal, createResource, For, Component } from 'solid-js';
import baseUrl from '../baseUrl';
import AddNewIngredient from '../components/AddNewIngredient';
import IngredientItem from '../components/IngredientItem';

// Define the type for Ingredient
interface Ingredient {
  ingredient_id: string;
  name: string;
  // Add other fields as needed
}

// Fetch ingredients with proper typing
const fetchIngredients = async (startFrom: number): Promise<Ingredient[]> => {
  const defaultLimit = 4;
  const response = await fetch(`${baseUrl}/ingredients?startFrom=${startFrom}&limit=${defaultLimit}`);
  const data = await response.json();
  return data.ingredients;
};

const Ingredients: Component = () => {
  const defaultLimit = 4;
  const [startFrom, setStartFrom] = createSignal(0);
  const [ingredients, { mutate }] = createResource<Ingredient[], number>(startFrom, fetchIngredients);

  const removeIngredient = (ingredientId: string) => {
    mutate((prevIngredients) =>
      prevIngredients?.filter((ingredient) => ingredient.ingredient_id !== ingredientId) || []
    );
  };

  const addIngredient = (newIngredient: Ingredient) => {
    mutate((prevIngredients) => [newIngredient, ...(prevIngredients || [])]);
  };

  return (
    <div class="min-h-full p-4 bg-beige flex flex-col gap-4 flex-1 py-5">
      <div class="grid justify-center items-center align-content-center grid-cols-3">
        <div></div>
        <div class="flex justify-center">
          <h1 class="text-4xl">Available ingredients</h1>
        </div>
        <div></div>
      </div>
      <div class="flex justify-stretch items-stretch mx-4">
        <input
          type="text"
          placeholder="Searching for an ingredient? Click here!"
          class="w-full rounded-3xl p-2 text-center border border-black"
        />
      </div>
      <div class="grid grid-cols-auto-fill gap-4" style={{ gridTemplateColumns: 'repeat(auto-fill, minmax(6rem, 1fr))' }}>
        <AddNewIngredient onAdd={addIngredient} />
        <For each={ingredients()}>
          {(ingredient) => (
            <IngredientItem
              ingredient={ingredient}
              onDelete={removeIngredient}
            />
          )}
        </For>
      </div>
    </div>
  );
};

export default Ingredients;
