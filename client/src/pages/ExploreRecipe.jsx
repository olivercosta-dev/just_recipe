import { ExploreRecipeCard as Recipe } from "../components/ExploreRecipeCard";
import carbonaraImage from '../assets/carbonara.webp';
import tikkaMasalaImage from '../assets/tikka.webp';
import stroganoffImage from '../assets/beef.webp';
import pekingDuckImage from '../assets/duck.webp';
import arrowNext from '../assets/arrow-next.svg'
import arrow from '../assets/arrow.png'
export default function Explore() {
  return (<>
    <div class='explore-page'>
      <div class="explore-card-container">
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
      <div className="explore-navigation">
        <div class="explore-previous-page">
          <img src={arrow} />
          <span>Prev</span>
        </div>
        <div class="explore-next-page">
          <span>Next</span>
          <img src={arrow} />
        </div>
      </div>
    </div>
  </>
  );
}


