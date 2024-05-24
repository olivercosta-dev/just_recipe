import { createSignal, createResource, For } from 'solid-js';
import { styled } from 'solid-styled-components';
import baseUrl from '../baseUrl';
import AddNewUnit from '../components/AddNewUnit';
import UnitItem from '../components/UnitItem';

const fetchUnits = async (startFrom) => {
    const defaultLimit = 4;
    const response = await fetch(baseUrl + '/units?' + `startFrom=${startFrom}&` + `limit=${defaultLimit}`);
    const data = await response.json();
    return data.units;
};

const UnitsPageContainer = styled.div`
    min-height: 100%;
    padding: 1rem;
    background-color: var(--beige);
    display: flex;
    flex-direction: column;
    gap: 1rem;
    flex: 1;
`;

const UnitsHeader = styled.div`
    display: grid;
    justify-content: center;
    align-items: center;
    align-content: center;
    grid-template-columns: 1fr minmax(5rem, 13rem) 1fr;
    text-wrap: nowrap;
`;

const AvailableUnits = styled.div`
    display: flex;
    justify-content: center;
`;

const HeaderTitle = styled.h1`
    font-size: var(--font-500);
`;

const UnitsSearch = styled.div`
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

const UnitsGrid = styled.div`
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(6rem, 1fr));
    gap: 1rem;
`;

export default function UnitsPage() {
    const defaultLimit = 4;
    const [startFrom, setStartFrom] = createSignal(0);
    const [units, { mutate }] = createResource(startFrom, fetchUnits);

    const removeUnit = (unitId) => {
        mutate((prevUnits) =>
            prevUnits.filter((unit) => unit.unit_id !== unitId)
        );
    };

    const addUnit = (newUnit) => {
        mutate((prevUnits) => [newUnit, ...prevUnits]);
    };

    return (
        <UnitsPageContainer>
            <UnitsHeader>
                <div></div>
                <AvailableUnits>
                    <HeaderTitle>Available units</HeaderTitle>
                </AvailableUnits>
                <div></div>
            </UnitsHeader>
            <UnitsSearch>
                <SearchInput type='text' placeholder='Searching for an unit? Click here!' />
            </UnitsSearch>
            <UnitsGrid>
                <AddNewUnit onAdd={addUnit} />
                <For each={units()}>
                    {(unit, index) => (
                        <UnitItem
                            unit={unit}
                            onDelete={removeUnit}
                        />
                    )}
                </For>
            </UnitsGrid>
        </UnitsPageContainer>
    );
}
