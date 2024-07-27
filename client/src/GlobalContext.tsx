import { createContext, useContext, JSX } from 'solid-js';
import { IngredientsProvider, useIngredients } from './IngredientsProvider';
import { UnitsProvider, useUnits } from './UnitsProvider';

interface GlobalContextType {
  ingredients: ReturnType<typeof useIngredients>;
  units: ReturnType<typeof useUnits>;
}

const GlobalContext = createContext<GlobalContextType | undefined>(undefined);

export function GlobalProvider(props: { children: JSX.Element }) {
  return (
    <IngredientsProvider>
      <UnitsProvider>
        <GlobalContext.Provider value={{ ingredients: useIngredients(), units: useUnits() }}>
          {props.children}
        </GlobalContext.Provider>
      </UnitsProvider>
    </IngredientsProvider>
  );
}

export function useGlobalContext() {
  const context = useContext(GlobalContext);
  if (!context) {
    throw new Error('useGlobalContext must be used within a GlobalProvider');
  }
  return context;
}
