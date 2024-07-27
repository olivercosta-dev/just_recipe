import { Component, createResource, createSignal, For } from 'solid-js';
import { ExploreRecipeCard } from "../components/ExploreRecipeCard";
import carbonaraImage from '../assets/carbonara.webp';
import tikkaMasalaImage from '../assets/tikka.webp';
import stroganoffImage from '../assets/beef.webp';
import pekingDuckImage from '../assets/duck.webp';
import arrowNext from '../assets/arrow-next.svg';
import arrow from '../assets/arrow.png';
import { GetRecipesResponse, Recipe } from '../interfaces';
import baseUrl from '../baseUrl';



const Explore: Component = () => {
  const [limit, setLimit] = createSignal<number>(4);
  const [startFrom, setStartFrom] = createSignal<number>(0);
  const fetchRecipes = async (): Promise<GetRecipesResponse> => {
    let query = '';
    query += 'start_from=' + startFrom().toString();
    query += `&limit=${limit().toString()}`
    const response = await fetch(`${baseUrl}/recipes?${query}`);
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }
    return response.json();
  }
  const [getRecipesResponse, { refetch }] = createResource<GetRecipesResponse>(fetchRecipes);

  return (
    <>
      <div class='p-3 bg-mid-beige min-h-[100lvh]'>
        <input type="text"
          placeholder='Want something specific?'
          class="
          w-full rounded-3xl 
          py-1 px-3 
          text-center 
          border border-black 
          max-w-96 
          mx-auto
          mb-5"
        />
        <div class="flex flex-col gap-4">
          {getRecipesResponse.loading && <div>Loading...</div>}
          {getRecipesResponse.error && <div>Error loading recipes</div>}
          <For each={getRecipesResponse()?.recipes}>
            {(recipe: Recipe, index) => (
              <ExploreRecipeCard
                image={carbonaraImage}
                name={recipe.name}
                description={recipe.description}
                recipe_id={recipe.recipe_id}
              />
            )}
          </For>
        </div>
        <div class="flex justify-between items-center m-x-4 mt-3">
          <div class="flex gap-1 items-center hover:cursor-pointer" onClick={() => {
            if (getRecipesResponse()?.previous_start_from !== null) {
              setStartFrom(getRecipesResponse()!.previous_start_from!);
              refetch();
            }
          }}
          >
            <img src={arrow} alt="Previous" class='m-h-6 -scale-x-100' />
            <span>Prev</span>
          </div>

          <div class="flex gap-1 items-center hover:cursor-pointer" onClick={() => {
            if (getRecipesResponse()?.next_start_from !== null) {
              setStartFrom(getRecipesResponse()!.next_start_from!);
              refetch();
            }
          }}
          >
            <span>Next</span>
            <img src={arrowNext} alt="Next" class='m-h-6' />
          </div>
        </div>
      </div >
    </>
  );
};

export default Explore;
