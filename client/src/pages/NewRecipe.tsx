import { createSignal, For, Show, Component } from 'solid-js';


interface Ingredient {
  id: number;
  name: string;
}

interface Unit {
  id: number;
  name: string;
}

interface RecipeIngredient {
  ingredient: string;
  ingredientId: number | null;
  unit: string;
  unitId: number | null;
  quantity: string;
}

interface Step {
  instruction: string;
  number: number;
}

const availableIngredients: Ingredient[] = [
  { id: 1, name: "Flour" },
  { id: 2, name: "Sugar" },
  { id: 3, name: "Salt" },
  { id: 4, name: "Butter" },
  { id: 5, name: "Milk" }
];

const availableUnits: Unit[] = [
  { id: 1, name: "g" },
  { id: 2, name: "kg" },
  { id: 3, name: "ml" },
  { id: 4, name: "l" },
  { id: 5, name: "cup" },
  { id: 6, name: "tsp" },
  { id: 7, name: "tbsp" }
];

const NewRecipePage: Component = () => {
  const [name, setName] = createSignal<string>("");
  const [description, setDescription] = createSignal<string>("");
  const [ingredients, setIngredients] = createSignal<RecipeIngredient[]>([
    { ingredient: "", ingredientId: null, unit: "", unitId: null, quantity: "" }
  ]);
  const [steps, setSteps] = createSignal<Step[]>([
    { instruction: "", number: 1 }
  ]);

  const [ingredientSuggestions, setIngredientSuggestions] = createSignal<Ingredient[]>([]);
  const [unitSuggestions, setUnitSuggestions] = createSignal<Unit[]>([]);

  const addIngredientField = () => {
    setIngredients([
      ...ingredients(),
      { ingredient: "", ingredientId: null, unit: "", unitId: null, quantity: "" }
    ]);
  };

  const addStepField = () => {
    setSteps([
      ...steps(),
      { instruction: "", number: steps().length + 1 }
    ]);
  };

  const handleIngredientChange = (index: number, field: string, value: string) => {
    const updatedIngredients = ingredients().map((ingredient, i) =>
      i === index ? { ...ingredient, [field]: value } : ingredient
    );
    setIngredients(updatedIngredients);
    if (field === "ingredient") {
      setIngredientSuggestions(
        availableIngredients.filter(ing => ing.name.toLowerCase().includes(value.toLowerCase()))
      );
    } else if (field === "unit") {
      setUnitSuggestions(
        availableUnits.filter(unit => unit.name.toLowerCase().includes(value.toLowerCase()))
      );
    }
  };

  const handleSuggestionClick = (
    index: number,
    field: string,
    value: string,
    idField: string,
    id: number
  ) => {
    const updatedIngredients = ingredients().map((ingredient, i) =>
      i === index ? { ...ingredient, [field]: value, [idField]: id } : ingredient
    );
    setIngredients(updatedIngredients);
    setIngredientSuggestions([]);
    setUnitSuggestions([]);
  };

  const handleStepChange = (index: number, value: string) => {
    const updatedSteps = steps().map((step, i) =>
      i === index ? { ...step, instruction: value } : step
    );
    setSteps(updatedSteps);
  };

  const handleSubmit = (e: Event) => {
    e.preventDefault();
    // Handle the form submission logic
    console.log({
      name: name(),
      description: description(),
      ingredients: ingredients(),
      steps: steps()
    });
  };

  return (
    <div class='flex flex-1 flex-col g-1 bg-mid-beige py-5 px-1'>
      <h1 class='text-center text-4xl'>New Recipe</h1>
      <form onSubmit={handleSubmit}>
        <div class='flex flex-col gap-2 mb-4'>
          <label class='text-xl fw-bold text-black' for="name">Name</label>
          <input
            id="name"
            type="text"
            value={name()}
            onInput={(e) => setName((e.target as HTMLInputElement).value)}
            class='p-4 rounded-3xl border-2 border-light-beige text-black focus:outline-none focus:border-dark-beige'
          />
        </div>
        <div class='flex flex-col gap-2 mb-4'>
          <label class='text-xl fw-bold text-black' for="description">Description</label>
          <textarea
            id="description"
            value={description()}
            onInput={(e) => setDescription((e.target as HTMLTextAreaElement).value)}
            rows="4"
            class='p-4 rounded-3xl border-2 border-dark-beige bg-light-beige text-black resize-none fcous:outline-none focus:border-dark-beige'
          />
        </div>
        <div class='flex flex-col gap-2 mb-4'>
          <label class='text-xl fw-bold text-black'>Ingredients</label>
          <For each={ingredients()}>
            {(ingredient, index) => (
              <div class='flex flex-col gap-2 mb4'>
                <input
                  type="text"
                  placeholder="Ingredient"
                  value={ingredient.ingredient}
                  onInput={(e) => handleIngredientChange(index(), "ingredient", (e.target as HTMLInputElement).value)}
                  class='p-4 rounded-3xl border-2 border-light-beige text-black focus:outline-none focus:border-dark-beige'
                />
                <div class='relative -mt-4 -mb-4'>
                  <Show when={ingredientSuggestions().length > 0}>
                    <div class='absolute z-50 bg-white border-zinc-200 border-1 rounded-3xl p-0 m-0 list-none'>
                      <For each={ingredientSuggestions()}>
                        {(suggestion) => (
                          <li class='p-4 cursor-pointer hover:bg-mid-beige' onClick={() => handleSuggestionClick(index(), "ingredient", suggestion.name, "ingredientId", suggestion.id)}>
                            {suggestion.name}
                          </li>
                        )}
                      </For>
                    </div>
                  </Show>
                </div>
                <input
                  type="text"
                  placeholder="Unit"
                  value={ingredient.unit}
                  onInput={(e) => handleIngredientChange(index(), "unit", (e.target as HTMLInputElement).value)}
                  class='p-4 rounded-3xl border-2 border-light-beige text-black focus:outline-none focus:border-dark-beige'
                />
                <div class='relative -mt-4 -mb-4'>
                  <Show when={unitSuggestions().length > 0}>
                    <div class='absolute z-50 bg-white border-zinc-200 border-1 rounded-3xl p-0 m-0 list-none'>
                      <For each={unitSuggestions()}>
                        {(suggestion) => (
                          <li class='p-4 cursor-pointer hover:bg-mid-beige' onClick={() => handleSuggestionClick(index(), "unit", suggestion.name, "unitId", suggestion.id)}>
                            {suggestion.name}
                          </li>
                        )}
                      </For>
                    </div>
                  </Show>
                </div>
                <input
                  type="number"
                  placeholder="Quantity"
                  value={ingredient.quantity}
                  onInput={(e) => handleIngredientChange(index(), "quantity", (e.target as HTMLInputElement).value)}
                  class='p-4 rounded-3xl border-2 border-light-beige text-black focus:outline-none focus:border-dark-beige'

                />
              </div>
            )}
          </For>
          <button type="button"
            onClick={addIngredientField}
            class="px-4 py-8 border-0 rounded-3xl bg-dark-beige text-white text-base cursor-pointer transition ease duration-300 hover:bg-dark-beige max-[768px]:p-4 max-[768px]:text-base" >
            Add Ingredient</button>
        </div>
        <div class='flex flex-col gap-2 mb-4'>
          <label class='text-xl fw-bold text-black'>Steps</label>
          <For each={steps()}>
            {(step, index) => (
              <div class='flex flex-col gap-2 mb-4'>
                <label class='text-xl fw-bold text-black'>Step {index() + 1}</label>
                <textarea
                  value={step.instruction}
                  onInput={(e) => handleStepChange(index(), (e.target as HTMLTextAreaElement).value)}
                  rows="2"
                  class='p-4 rounded-3xl border-2 border-dark-beige bg-light-beige text-black resize-none fcous:outline-none focus:border-dark-beige'
                />
              </div>
            )}
          </For>
          <button type="button"
            onClick={addStepField}
            class="px-4 py-8 border-0 rounded-3xl bg-dark-beige text-white text-base cursor-pointer transition ease duration-300 hover:bg-dark-beige max-[768px]:p-4 max-[768px]:text-base" >Add Step</button>
        </div>
        <button
          type="submit"
          class="px-4 py-8 border-0 rounded-3xl bg-dark-beige text-white text-base cursor-pointer transition ease duration-300 hover:bg-dark-beige max-[768px]:p-4 max-[768px]:text-base" >
          Save Recipe</button>
      </form>
    </div>
  );
};

export default NewRecipePage;
