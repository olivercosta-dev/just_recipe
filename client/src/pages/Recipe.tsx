import { createSignal, For, onMount, Component, createResource } from 'solid-js';
import { useIngredients } from '../IngredientsProvider';
import { Select, createOptions } from '@thisbeyond/solid-select';
import { useUnits } from '../UnitsProvider';
import { Ingredient, CompactRecipeIngredient, RecipeStep, Unit } from '../interfaces';
import { createStore } from 'solid-js/store';
import baseUrl from '../baseUrl';
import { useParams } from '@solidjs/router';

const fetchRecipeData = async (id: number) => {
    const response = await fetch(baseUrl + `/recipes/${id}`);
    return response.json();
};

const Recipe: Component = () => {
    const params = useParams();

    const [recipe] = createResource(() => parseInt(params.id), fetchRecipeData);

    return (
        <div class='bg-japanese-light-blue min-h-[100dvh] mx-3 mt-2 rounded-t-3xl'>
            <div class='bg-white mt-3 rounded-t-3xl p-2 mx-2'>
                <h1 class='text-2xl w-fit mx-auto underline'>
                    {recipe() ? recipe().name : 'Loading...'}
                </h1>
            </div>
            <div class='mt-2'>
                <h2 class='text-xl w-fit mx-auto text-center'><span class='text-white bg-bento-red rounded-3xl px-2 py-0.5'>Description</span></h2>
            </div>
            <div class='px-2'>
                <p
                    class='box-border rounded-bl-2xl rounded-r-2xl rounded bg-white w-full my-3 p-3'
                >
                    {recipe() ? recipe().description : 'Loading...'}
                </p>
            </div>
            <div class='mt-2'>
                <h2 class='text-xl w-fit mx-auto text-center'>
                    <span class='text-white bg-bento-red rounded-3xl px-2 py-0.5'>Ingredients</span>
                </h2>
            </div>
            <div class='my-5'>
                {
                    recipe() ?
                        <For each={recipe().ingredients}>
                            {(recipeIngredient, index) => (<div class='flex gap-2 px-2 mt-3 justify-center'>
                                <div
                                    class='i-select px-3 text-white bg-orbit-blue rounded-2xl'
                                >{recipeIngredient.ingredient.singular_name}</div>
                                <div
                                    class='u-select px-3 text-black bg-white rounded-2xl'
                                >{recipeIngredient.unit.singular_name}</div>
                                <div
                                    class='text-white bg-forest-green rounded-2xl px-2'>
                                    {recipeIngredient.quantity}
                                </div>
                            </div>
                            )}
                        </For> : 'Loading...'
                }
            </div>
            <div class='mt-2'>
                <h2 class='text-xl w-fit mx-auto text-center'>
                    <span class='text-white bg-bento-red rounded-3xl px-2 py-0.5'>Steps</span>
                </h2>
            </div>
            <div class='my-5'>
                {
                    recipe() ?
                        <For each={recipe().steps}>
                            {
                                (step, index) => (
                                    <div class='flex flex-col justify-center my-5'>
                                        <div class='flex justify-center items-center underline underline-offset-2 mb-2'>
                                            <span class='text-lg'>
                                                Step {step.step_number}
                                            </span>
                                        </div>
                                        <div class='flex justify-center'>
                                            <p
                                                class='bg-white w-full p-3'
                                            >
                                                {step.instruction}
                                            </p>
                                        </div>
                                    </div>
                                )
                            }
                        </For> : 'Loading...'
                }
            </div>
        </div>
    );
};

export default Recipe;
