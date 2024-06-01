import { Component } from 'solid-js';

interface ExploreRecipeCardProps {
  image: string;
  name: string;
  description: string;
}

export const ExploreRecipeCard: Component<ExploreRecipeCardProps> = ({ image, name, description }) => {
  return (
    <div class="
      explore-card 
      flex items-stretch justify-center
      gap-8
      py-3
      px-5
      text-[#F5F5F5]
      rounded-3xl
      max-h-36
      overflow-hidden
      text-ellipsis
      border-solid
      border-black
      border
      shadow-[-1px_2px_0px_0px_rgba(0,0,0,0.75)]
      odd:bg-gradient-to-l
      even:bg-gradient-to-r
      from-red-700 to-red-500 
    
    ">
      <img class="block aspect-square max-h-32 rounded-3xl" src={image} alt={name} />
      <div class="flex flex-col flex-1 justify-between overflow-hidden text-ellipsis">
        <span class="text-base font-semibold underline underline-offset-2">{name}</span>
        <span class="flex-1 text-xs text-ellipsis overflow-hidden">
          {description}
        </span>
      </div>
    </div>
  );
};
