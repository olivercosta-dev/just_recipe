import { Component } from 'solid-js';
import { Router } from '@solidjs/router';
import Navbar from './components/Navbar';
import routes from './routes';

// Importing the components for routing
import Explore from './pages/Explore';
import MyProfile from './pages/MyProfile';
import Friends from './pages/Friends';
import Saved from './pages/Saved';
import NewRecipe from './pages/NewRecipe';
import Home from './pages/Home';

// Importing the logos (assuming they are used in the Navbar or elsewhere)
import exploreLogo from './assets/explore-logo.svg';
import friendsLogo from './assets/friends-logo.svg';
import myProfileLogo from './assets/my-profile-logo.svg';
import newRecipeLogo from './assets/new-recipe-logo.svg';
import savedLogo from './assets/saved-logo.svg';

const App: Component = () => {
  return (
    <>
      <Navbar />
      <Router>
        {routes}
      </Router>
    </>
  );
};

export default App;
