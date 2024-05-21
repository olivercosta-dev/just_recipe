import { ExploreRecipeCard as Recipe } from '../components/ExploreRecipeCard';
import carbonaraImage from '../assets/carbonara.webp';
import tikkaMasalaImage from '../assets/tikka.webp';
import stroganoffImage from '../assets/beef.webp';
import pekingDuckImage from '../assets/duck.webp';
import arrowNext from '../assets/arrow-next.svg';
import arrow from '../assets/arrow.png';
import { createSignal, createResource, Switch, Match, Show } from 'solid-js';
import addNewIcon from '../assets/add-new-icon.svg'
import baseUrl from '../baseUrl';
import carrotIcon from '../assets/ingredient_icons/carrot.svg'
const fetchIngredients = async (startFrom) => {
    const defaultLimit = 4;
    const response = await fetch(baseUrl + '/ingredients?' + `startFrom=${startFrom}&` + `limit=${defaultLimit}`);
    const data = await response.json();
    return data.ingredients;
}
export default function Ingredients() {
    const defaultLimit = 4;

    const [startFrom, setStartFrom] = createSignal(0);
    const [ingredients] = createResource(startFrom, fetchIngredients)
    return (<>
        <div class='ingredients-page'>
            <div class='ingredients-header'>
                <div></div>
                <div class=''>
                    <h1>Available ingredients</h1>
                </div>
                <div class='add-new-ingredient'>
                    <span>Add new</span>
                    <img src={addNewIcon} />
                </div>
            </div>
            <div class='ingredients-search'>
                <input type='text' placeholder='Searching for an ingredient? Click here!'/>
            </div>
            <div class='ingredients-grid'>
                <div class='available-ingredient'>
                    <img src={carrotIcon} alt="" />
                    <span class='ingredient-name'>Carrot</span>
                </div>
                <div class='available-ingredient'>
                    <img src={carrotIcon} alt="" />
                    <span class='ingredient-name'>Onion</span>
                </div>
                <div class='available-ingredient'>
                    <img src={carrotIcon} alt="" />
                    <span class='ingredient-name'>Carrot</span>
                </div>
                <div class='available-ingredient'>
                    <img src={carrotIcon} alt="" />
                    <span class='ingredient-name'>Potato</span>
                </div>
            </div>
        </div>
    </>
    );
}


