import { createSignal, createContext, useContext, JSX } from 'solid-js';
import baseUrl from './baseUrl';
import { Ingredient } from './interfaces';

interface IngredientsContextType {
  ingredients: () => Ingredient[];
  fetchIngredients: () => Promise<void>;
}

export const IngredientsContext = createContext<IngredientsContextType | undefined>(undefined);

export function IngredientsProvider(props: { children: JSX.Element }) {
  const [ingredients, setIngredients] = createSignal<Ingredient[]>([]);

  const fetchIngredients = async () => {
    try {
      const response = await fetch(`${baseUrl}/ingredients/all`);
      const data: Ingredient[] = await response.json();
      setIngredients(data);
    } catch (error) {
      console.error('Failed to fetch ingredients:', error);
    }
  };

  return (
    <IngredientsContext.Provider value={{ ingredients, fetchIngredients }}>
      {props.children}
    </IngredientsContext.Provider>
  );
}

export function useIngredients() {
  const context = useContext(IngredientsContext);
  if (!context) {
    throw new Error('useIngredients must be used within an IngredientsProvider');
  }
  return context;
}
