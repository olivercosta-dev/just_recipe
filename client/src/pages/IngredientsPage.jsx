import { createSignal, createResource, For } from 'solid-js';
import { styled } from 'solid-styled-components';
import baseUrl from '../baseUrl';
import AddNewIngredient from '../components/AddNewIngredient';
import IngredientItem from '../components/IngredientItem';
import { useIngredients } from '../IngredientsProvider';

const fetchIngredients = async () => {
    const defaultLimit = 4;
    const response = await fetch(baseUrl + '/ingredients/all');
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
    const { ingredients } = useIngredients();

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
                <AddNewIngredient />
                <For each={ingredients()}>
                    {(ingredient, index) => (
                        <IngredientItem
                            ingredient={ingredient}
                        />
                    )}
                </For>
            </IngredientsGrid>
        </IngredientsPageContainer>
    );
}
