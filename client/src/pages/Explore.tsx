import { Component } from 'solid-js';
import { ExploreRecipeCard as Recipe } from "../components/ExploreRecipeCard";
import carbonaraImage from '../assets/carbonara.webp';
import tikkaMasalaImage from '../assets/tikka.webp';
import stroganoffImage from '../assets/beef.webp';
import pekingDuckImage from '../assets/duck.webp';
import arrowNext from '../assets/arrow-next.svg';
import arrow from '../assets/arrow.png';

const Explore: Component = () => {
  return (
    <>
      <div class='explore-page p-1 bg-mid-beige min-h-full'>
        <div class="flex flex-col gap-1">
          <Recipe
            image={carbonaraImage}
            name="Spaghetti Carbonara"
            description="A classic Italian pasta dish with a creamy egg-based sauce, crispy pancetta, and grated Parmesan cheese."
          />
          <Recipe
            image={tikkaMasalaImage}
            name="Chicken Tikka Masala"
            description="Marinated chicken chunks cooked in a spiced tomato cream sauce, a popular Indian dish."
          />
          <Recipe
            image={stroganoffImage}
            name="Beef Stroganoff"
            description="Sautéed beef strips in a creamy mushroom and onion sauce, served over egg noodles."
          />
          <Recipe
            image={pekingDuckImage}
            name="Peking Duck"
            description="Roasted duck with crispy skin, served with hoisin sauce, pancakes, and scallions."
          />
        </div>
        <div class="flex justify-center items-center m-x-4">
          <div class="flex gap-1 items-center">
            <img src={arrow} alt="Previous" class='m-h-6 -scale-x-100'/>
            <span>Prev</span>
          </div>
          <div class="flex gap-1 items-center">
            <span>Next</span>
            <img src={arrowNext} alt="Next" class='m-h-6'/>
          </div>
        </div>
      </div>
    </>
  );
};

export default Explore;
