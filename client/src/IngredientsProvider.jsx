import {setIngredients, ingredients} from './store.js'
import { createContext, useContext } from 'solid-js';
const IngredientsContext = createContext();
import baseUrl from './baseUrl.js';
export function IngredientsProvider(props) {
    const fetchIngredients = async () => {
        try {
            const response = await fetch(`${baseUrl}/ingredients/all`);
            const data = await response.json();
            setIngredients(data);
        } catch (error) {
            console.error("Failed to fetch ingredients:", error);
        }
    };

    return (
        <IngredientsContext.Provider value={{ ingredients, fetchIngredients }}>
            {props.children}
        </IngredientsContext.Provider>
    );
}

export function useIngredients() {
    return useContext(IngredientsContext);
}