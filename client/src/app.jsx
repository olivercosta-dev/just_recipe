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
import { Router, Route } from '@solidjs/router';
function App() {

  // This is gonna be the Home Page for now.
  return (<>
    <Navbar></Navbar>
    <Router>
      {routes}
    </Router>
  </>
  );
}

export default App;
