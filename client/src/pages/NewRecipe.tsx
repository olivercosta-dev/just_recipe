import { createSignal, For, onMount, Component } from 'solid-js';
import { useIngredients } from '../IngredientsProvier';
import { Select, createOptions } from '@thisbeyond/solid-select';
import { useUnits } from '../UnitsProvider';
import { Ingredient, Unit } from '../interfaces';
import { createStore } from 'solid-js/store';
import baseUrl from '../baseUrl';

interface RecipeIngredient {
  ingredient_id?: number;
  unit_id?: number;
  quantity: string;
}

interface RecipeStep {
  step_number: number,
  instruction: string
}
const NewRecipePage: Component = () => {
  const { ingredients: allIngredients, fetchIngredients } = useIngredients();
  const { units: allUnits, fetchUnits } = useUnits();

  onMount(() => {
    fetchIngredients();
    fetchUnits();
  });

  const [name, setName] = createSignal('');
  const [description, setDescription] = createSignal("");

  const [recipeIngredients, setRecipeIngredients] = createStore<RecipeIngredient[]>([
    {
      ingredient_id: 0,
      unit_id: 0,
      quantity: '0'
    }
  ]);
  const [recipeSteps, setRecipeSteps] = createStore<RecipeStep[]>([
    {
      step_number: 1,
      instruction: ""
    }
  ]);
  const allIngredientsOptions = createOptions(allIngredients, { key: 'singular_name' });
  const allUnitOptions = createOptions(allUnits, { key: 'singular_name' });

  const addNewRecipeIngredient = () => {
    setRecipeIngredients([...recipeIngredients, { ingredient_id: 0, unit_id: 0, quantity: '0' }]);
  };
  const addNewRecipeStep = () => {
    const newStepNumber = recipeSteps.length + 1;
    setRecipeSteps([...recipeSteps, { step_number: newStepNumber, instruction: "" }]);
  };
  const setRecipeStep = (index: number, newInstruction: string) => {
    setRecipeSteps(index, 'instruction', newInstruction);
  };
  const removeRecipeStep = (index: number) => {
    setRecipeSteps(
      recipeSteps
        .map((step, currentIndex) => {
          if (currentIndex === index) {
            // Skip the step to be removed
            return null;
          } else if (currentIndex > index) {
            // Decrease the step number for steps after the removed step
            return { ...step, step_number: currentIndex };
          } else {
            // Return the step as is for steps before the removed step
            return step;
          }
        })
        .filter(step => step !== null) // Remove the null step
    );
    console.log(recipeSteps);
  };
  const setIngredientId = (index: number, newIngredientId: number) => {
    setRecipeIngredients(index, 'ingredient_id', newIngredientId);
  };

  const setUnitId = (index: number, newUnitId: number) => {
    setRecipeIngredients(index, 'unit_id', newUnitId);
  };
  const [saveButtonText, setSaveButtonText] = createSignal<string>('Save');
  const handleSaveRecipe = async () => {
    try {
      const response = await fetch(`${baseUrl}/recipes`, {
        method: 'POST',
        headers: {
          'Content-type': 'application/json'
        },
        body: JSON.stringify(
          {
            'name': name(),
            'description': description(),
            'ingredients': recipeIngredients,
            'steps': recipeSteps,
          }
        )
      })
      if (response.ok) {
        setSaveButtonText('Added Successfully');
      } else {
        setSaveButtonText('Failed to Add');
      }
    } catch (error) {
      setSaveButtonText('Failed to Add');
    }
    setTimeout(() => {
      setSaveButtonText('Save');
    }, 1000);
  }
  return (
    <div class='bg-japanese-light-blue min-h-[100dvh] mx-3 mt-2 rounded-t-3xl'>
      <div class='bg-foggy-gray mt-3 rounded-t-3xl p-2 mx-2'>
        <h1 class='text-2xl w-fit mx-auto underline'>New Recipe</h1>
      </div>
      <div class='mt-2'>
        <h2 class='text-xl w-fit mx-auto text-center'>Give your recipe a <span class='text-white bg-bento-red rounded-3xl px-2 py-0.5'>name!</span></h2>
      </div>
      <input
        class='w-full bg-bento-red text-white underline p-3 my-3 text-center placeholder:text-white placeholder:italic'
        type='text'
        placeholder='Here goes the name...'
        onChange={(e) => setName(e.currentTarget.value)}
        value={name()}
      />
      <div class='mt-2'>
        <h2 class='text-xl w-fit mx-auto text-center'>Write a short <span class='text-white bg-bento-red rounded-3xl px-2 py-0.5'>description!</span></h2>
      </div>
      <div class='px-2'>
        <textarea
          name='description'
          placeholder='Here goes the description...'
          onChange={(e) => setDescription(e.currentTarget.value)}
          value={description()}
          class='box-border rounded-bl-2xl rounded-r-2xl rounded bg-foggy-gray w-full my-3 p-3 h-[7rem]'
        />
      </div>
      <div class='mt-2'>
        <h2 class='text-xl w-fit mx-auto text-center'>
          Now let's start adding the <br /> <span class='text-white bg-bento-red rounded-3xl px-2 py-0.5'>ingredients...</span>
        </h2>
      </div>
      <div class='my-5'>
        <For each={recipeIngredients}>
          {(recipeIngredient, index) => (
            <IngredientSelect
              ingredientOptions={allIngredientsOptions}
              unitOptions={allUnitOptions}
              onUnitChange={(newUnitId: number) => setUnitId(index(), newUnitId)}
              onIngredientChange={(newIngredientId: number) => setIngredientId(index(), newIngredientId)}
            />
          )}
        </For>
      </div>
      <button class='mx-auto block my-5 bg-[#3A9DFB] text-white rounded-2xl px-2 py-1' onClick={addNewRecipeIngredient}>
        Add New
      </button>
      <h2 class='text-2xl w-fit mx-auto text-center my-8'>So far so good!</h2>
      <h2 class='text-xl w-fit mx-auto text-center'>Now it's time to bring it all together with the <span class='text-white bg-bento-red rounded-3xl px-2 py-0.5'>instructions!</span></h2>
      <For each={recipeSteps}>
        {
          (step, index) => (
            <div class='flex flex-row justify-center my-5   min-h-[5rem]'>
              <div class='rounded-s-full flex justify-center items-center mr-2 underline underline-offset-2'><span>Step {step.step_number}</span></div>
              <div class='flex'>
                <textarea
                  name='description'
                  placeholder='Here goes the instruction...'
                  value={step.instruction}
                  onChange={(event) => setRecipeStep(index(), event.currentTarget.value)}
                  class='resize-none bg-white rounded-s-2xl ps-2 pt-2'
                />
                <button onClick={() => removeRecipeStep(index())}
                  class='bg-red-500 rounded-e-full px-3 flex justify-center items-center text-white' ><span>Remove</span></button>
              </div>
            </div>
          )
        }
      </For>
      <button class='mx-auto block my-5 bg-[#3A9DFB] text-white rounded-2xl px-2 py-1' onClick={addNewRecipeStep}>
        Add New
      </button>
      <h2 class='text-xl w-fit mx-auto text-center my-5 italic'>Ready to save?</h2>
      <div class='flex mt-5 justify-around mb-10'>
        <button class='px-5 bg-red-500 text-white rounded-2xl'>Cancel</button>
        <button class='px-6 bg-green-600 text-white rounded-2xl' onClick={handleSaveRecipe}>{saveButtonText()}</button>
      </div>
    </div>
  );
};

const IngredientSelect: Component<{ ingredientOptions: any; unitOptions: any; onUnitChange: any; onIngredientChange: any }> = ({ ingredientOptions, unitOptions, onUnitChange, onIngredientChange }) => {
  const [ingredientId, setIngredientId] = createSignal<number>(-1);
  const [unitId, setUnitId] = createSignal<number>(-1);
  const [quantity, setQuantity] = createSignal('0');

  const handleIngredientChange = (ingredient: Ingredient) => {
    if (ingredient.ingredient_id) {
      const id = parseInt(ingredient.ingredient_id); // Ensure it's a number
      setIngredientId(id);
      onIngredientChange(id);
    }
  };

  const handleUnitChange = (unit: Unit) => {
    if (unit.unit_id) {
      const id = parseInt(unit.unit_id); // Ensure it's a number
      setUnitId(id);
      onUnitChange(id);
    }
  };

  return (
    <div class='flex gap-2 px-2 mt-3 justify-center' data-ingredient-id={ingredientId()} data-unit-id={unitId()}>
      <Select
        {...ingredientOptions}
        onChange={handleIngredientChange}
        class='i-select px-3 text-white bg-bento-red rounded-2xl'
        placeholder='ingredient'
        emptyPlaceholder=''
      />
      <Select
        {...unitOptions}
        onChange={handleUnitChange}
        class='u-select px-3 text-black bg-white rounded-2xl'
        placeholder='unit'
        emptyPlaceholder=''
      />
      <div class='text-white bg-forest-green rounded-2xl px-2'>
        <input
          type='text'
          name='quantity'
          placeholder='quantity'
          class='bg-transparent text-white placeholder-white border-0 focus:outline-none max-w-[7ch]'
          value={quantity()}
          onInput={(e) => setQuantity(e.currentTarget.value)}
        />
      </div>
    </div>
  );
};

export default NewRecipePage;
