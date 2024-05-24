import Explore from './pages/ExploreRecipe';
import MyProfile from './pages/MyProfile'
import Friends from './pages/Friends';
import Saved from './pages/Saved';
import NewRecipe from './pages/NewRecipe';
import Navbar from './components/Navbar';
import exploreLogo from './assets/explore-logo.svg'
import friendsLogo from './assets/friends-logo.svg'
import myProfileLogo from './assets/my-profile-logo.svg'
import newRecipeLogo from './assets/new-recipe-logo.svg'
import savedLogo from './assets/saved-logo.svg'
import HomePage from './pages/HomePage';
import routes from './routes';
import { Router } from '@solidjs/router';
import { createStore } from 'solid-js/store';
import baseUrl from './baseUrl';
import { createResource, onMount } from 'solid-js';
import { createContext, useContext } from "solid-js";
import { ingredients, setIngredients } from "./store";
import { IngredientsProvider, useIngredients } from './IngredientsProvider';

function App() {
  const { fetchIngredients } = useIngredients();
  onMount(() => {
    fetchIngredients()
  })
  return (<>
    <Navbar></Navbar>
    <Router>
      {routes}
    </Router>
  </>
  );
}

export default App;
