import { createSignal, For, Show, Component, onMount } from 'solid-js';
import { useIngredients } from '../IngredientsProvier';
import { Select, createOptions } from '@thisbeyond/solid-select';
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


const NewRecipePage: Component = () => {
  const [name, setName] = createSignal('');
  const [description, setDescription] = createSignal("Omg I love Vegan meat, it’s literally the best. It literally tastes like awesomeness. Though I’ve only had it once, it really blew my mind. The texture and flavor were so close to real meat that I couldn't believe it.");
  const {ingredients: allIngredients} = useIngredients();
  const allIngredientsOptions = createOptions(allIngredients(), {key: "singular_name"});
  return <div class='bg-japanese-light-blue min-h-[100dvh] mx-3 mt-2 rounded-t-3xl'>
    <div class='bg-foggy-gray mt-3 rounded-t-3xl p-2 mx-2'>
      <h1 class='text-2xl w-fit mx-auto underline'>New Recipe</h1>
    </div>
    <div class='mt-2'>
      <h2 class='text-xl w-fit mx-auto text-center'>Give your recipe a <span class='text-white bg-bento-red rounded-3xl px-2 py-0.5'>name!</span></h2>
    </div>
    <input class='w-full bg-bento-red text-white underline p-3 my-3 text-center placeholder:text-white placeholder:italic'
      type="text" placeholder='Here goes the name...'
      onChange={(e) => setName(e.currentTarget.value)}
      value={name()} />
    <div class='mt-2'>
      <h2 class='text-xl w-fit mx-auto text-center'>Write a short <span class='text-white bg-bento-red rounded-3xl px-2 py-0.5'>description!</span></h2>
    </div>
    <div class='px-2'>
      <textarea name="description" 
        placeholder="Here goes the description..."
        onChange={(e) => setDescription(e.currentTarget.value)}
        value={description()}
        class='box-border rounded-bl-2xl rounded-r-2xl rounded bg-foggy-gray w-full my-3 p-3 h-[7rem]'
      />
    </div>
    <div class='mt-2'>
      <h2 class='text-xl w-fit mx-auto text-center'>Now let's start adding the <br /> <span class='text-white bg-bento-red rounded-3xl px-2 py-0.5'>ingredients...</span></h2>
    </div>
    <div class='flex gap-2 px-2 mt-3 justify-center'>
      <div class='text-white bg-bento-red rounded-2xl px-3'><Select {...allIngredientsOptions} /></div>
      <div  class='text-black bg-white rounded-2xl px-5'><span>unit</span></div>
      <div  class='text-white bg-forest-green rounded-2xl px-3'><span>quantity</span></div>
    </div>
  </div>

};

export default NewRecipePage;
