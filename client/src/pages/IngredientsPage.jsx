import { createSignal, createResource, For } from 'solid-js';
import { styled } from 'solid-styled-components';
import baseUrl from '../baseUrl';
import AddNewIngredient from '../components/AddNewIngredient';
import IngredientItem from '../components/IngredientItem';

const fetchIngredients = async (startFrom) => {
    const defaultLimit = 4;
    const response = await fetch(baseUrl + '/ingredients?' + `startFrom=${startFrom}&` + `limit=${defaultLimit}`);
    const data = await response.json();
    return data.ingredients;
};

const IngredientsPageContainer = styled.div`
    min-height: 100%;
    padding: 1rem;
    background-color: var(--beige);
    display: flex;
    flex-direction: column;
    gap: 1rem;
    flex: 1;
`;

const IngredientsHeader = styled.div`
    display: grid;
    justify-content: center;
    align-items: center;
    align-content: center;
    grid-template-columns: 1fr minmax(5rem, 13rem) 1fr;
    text-wrap: nowrap;
`;

const AvailableIngredients = styled.div`
    display: flex;
    justify-content: center;
`;

const HeaderTitle = styled.h1`
    font-size: var(--font-500);
`;

const IngredientsSearch = styled.div`
    display: flex;
    justify-content: stretch;
    align-items: stretch;
    margin-inline: 1rem;
`;

const SearchInput = styled.input`
    width: 100%;
    border-radius: 1rem;
    padding: 0.5rem;
    text-align: center;
    border: 1px solid black;
`;

const IngredientsGrid = styled.div`
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(6rem, 1fr));
    gap: 1rem;
`;

export default function IngredientsPage() {
    const defaultLimit = 4;
    const [startFrom, setStartFrom] = createSignal(0);
    const [ingredients, { mutate }] = createResource(startFrom, fetchIngredients);

    const removeIngredient = (ingredientId) => {
        mutate((prevIngredients) =>
            prevIngredients.filter((ingredient) => ingredient.ingredient_id !== ingredientId)
        );
    };

    const addIngredient = (newIngredient) => {
        mutate((prevIngredients) => [newIngredient, ...prevIngredients]);
    };

    return (
        <IngredientsPageContainer>
            <IngredientsHeader>
                <div></div>
                <AvailableIngredients>
                    <HeaderTitle>Available ingredients</HeaderTitle>
                </AvailableIngredients>
                <div></div>
            </IngredientsHeader>
            <IngredientsSearch>
                <SearchInput type='text' placeholder='Searching for an ingredient? Click here!' />
            </IngredientsSearch>
            <IngredientsGrid>
                <AddNewIngredient onAdd={addIngredient} />
                <For each={ingredients()}>
                    {(ingredient, index) => (
                        <IngredientItem
                            ingredient={ingredient}
                            onDelete={removeIngredient}
                        />
                    )}
                </For>
            </IngredientsGrid>
        </IngredientsPageContainer>
    );
}
