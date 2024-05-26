import { For, Component, Show, onMount, createSignal } from 'solid-js';
import AddNewIngredient from '../components/AddNewIngredient';
import IngredientItem from '../components/IngredientItem';
import { useIngredients } from '../IngredientsProvier';
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
    <div class="min-h-full p-4 bg-beige flex flex-col gap-4 flex-1 py-5 bg-mid-beige">
      <div class="">
          <h1 class="text-3xl text-center">Available ingredients</h1>
      </div>
      <div class="flex justify-stretch items-stretch mx-4 col-span-4">
        <input
          type="text"
          placeholder="Searching for an ingredient? Click here!"
          onInput={(e) => setSearchInput(e.currentTarget.value.toLowerCase())}
          class="w-full rounded-3xl py-1 px-3 text-center border border-black max-w-96 mx-auto"
        />
      </div>
      <div class="grid grid-cols-auto-fill gap-4 max-auto" style={{ 'grid-template-columns': 'repeat(auto-fill, minmax(6rem, 1fr))' }}>
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
