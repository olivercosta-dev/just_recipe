import { For, Component, Show, onMount, createSignal } from 'solid-js';
import AddNewIngredient from '../components/AddNewIngredient';
import IngredientItem from '../components/IngredientItem';
import { useIngredients } from '../IngredientsProvier';
import { Combobox } from "@kobalte/core/combobox";
import { Ingredient } from '../interfaces';

const Ingredients: Component = () => {
  const [searchInput, setSearchInput] = createSignal('');

  const { ingredients, fetchIngredients } = useIngredients();
  const filteredIngredients = (): Ingredient[] => {
    return ingredients().filter(ingredient => ingredient.singular_name.includes(searchInput()) || ingredient.plural_name.includes(searchInput()));
  }
  onMount(() => {
    fetchIngredients();
  });

  return (
    <div class="min-h-full p-4 bg-beige flex flex-col gap-4 flex-1 py-5">
      <div class="grid justify-center items-center align-content-center grid-cols-3">
        <div></div>
        <div class="flex justify-center">
          <h1 class="text-3xl text-center">Available ingredients</h1>
        </div>
        <div></div>
      </div>
      <div class="flex justify-stretch items-stretch mx-4">
        <input
          type="text"
          placeholder="Searching for an ingredient? Click here!"
          onInput={(e) => setSearchInput(e.currentTarget.value)}
          class="w-full rounded-3xl p-2 text-center border border-black"
        />
      </div>
      <div class="grid grid-cols-auto-fill gap-4" style={{ 'grid-template-columns': 'repeat(auto-fill, minmax(6rem, 1fr))' }}>
        <AddNewIngredient />
        <Show when={ingredients() === null || ingredients === undefined}>
          <div>Loading ingredients...</div>
        </Show>
        <For each={filteredIngredients()}>
          {(ingredient) => (
            <IngredientItem
              ingredient={ingredient}
              refetchIngredients={fetchIngredients}
            />
          )}
        </For>
      </div>
    </div>
  );
};

export default Ingredients;
