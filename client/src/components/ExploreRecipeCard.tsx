import { Link } from '@kobalte/core/*';
import { Component } from 'solid-js';

interface ExploreRecipeCardProps {
  image: string;
  name: string;
  description: string;
  recipe_id: number;
}

export const ExploreRecipeCard: Component<ExploreRecipeCardProps> = ({ image, name, description, recipe_id }) => {
  return (
    <a href={'/recipes/' + recipe_id}

    class="
      explore-card
      flex items-stretch justify-center
      gap-8
      py-3
      px-5
      rounded-3xl
      max-h-36
      overflow-hidden
      text-ellipsis
      border-solid
      border-black
      border
      bg-[#F87C6B]
    "
    >
      <img class="block aspect-square max-h-32 rounded-3xl" src={image} alt={name} />
      <div class="flex flex-col flex-1 justify-between overflow-hidden text-ellipsis ">
        <span class="text-base bg-white text-black rounded-full px-3 text-ellipsis overflow-hidden whitespace-nowrap">{name}</span>
        <span class="flex-1 text-xs text-ellipsis overflow-hidden text-black px-3 py-1">
          {description}
        </span>
      </div>
    </a>
  );
};
