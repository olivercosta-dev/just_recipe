import Navbar from './components/Navbar';

import routes from './routes';
import { Router } from '@solidjs/router';
import baseUrl from './baseUrl';
import { useIngredients } from './IngredientsProvider';

function App() {
 
  return (<>
    <Navbar />
    <Router>
      {routes}
    </Router>
  </>
  );
}

export default App;
