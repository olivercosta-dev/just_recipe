import { Component } from 'solid-js';

interface ExploreRecipeCardProps {
  image: string;
  name: string;
  description: string;
}

export const ExploreRecipeCard: Component<ExploreRecipeCardProps> = ({ image, name, description }) => {
  return (
    <div class="explore-card grid grid-cols-2 items-stretch justify-center p-1 bg-dark-beige rounded-md max-h-40 overflow-hidden">
      <img class="explore-card-image block aspect-square max-h-32 object-contain rounded-3xl" src={image} alt={name} />
      <div class="explore-card-info flex flex-col flex-1 justify-between">
        <span class="explore-card-name text-base">{name}</span>
        <span class="flex-1 text-sm">
          {description}
        </span>
      </div>
    </div>
  );
};
