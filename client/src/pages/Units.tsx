import { createSignal, createResource, For, Component } from 'solid-js';
import baseUrl from '../baseUrl';
import AddNewUnit from '../components/AddNewUnit';
import UnitItem from '../components/UnitItem';
import { Unit } from '../interfaces';

// Fetch units with proper typing
const fetchUnits = async (startFrom: number): Promise<Unit[]> => {
  const defaultLimit = 4;
  const response = await fetch(`${baseUrl}/units?startFrom=${startFrom}&limit=${defaultLimit}`);
  const data = await response.json();
  return data.units;
};

const Units: Component = () => {
  const defaultLimit = 4;
  const [startFrom, setStartFrom] = createSignal(0);
  const [units, { mutate }] = createResource<Unit[], number>(startFrom, fetchUnits);

  const removeUnit = (unitId: string) => {
    mutate((prevUnits) =>
      prevUnits?.filter((unit) => unit.unit_id !== unitId) || []
    );
  };

  const addUnit = (newUnit: Unit) => {
    mutate((prevUnits) => [newUnit, ...(prevUnits || [])]);
  };

  return (
    <div class="min-h-full p-4 bg-beige flex flex-col gap-4 flex-1">
      <div class="grid justify-center items-center align-content-center grid-cols-3">
        <div></div>
        <div class="flex justify-center">
          <h1 class="text-4xl">Available units</h1>
        </div>
        <div></div>
      </div>
      <div class="flex justify-stretch items-stretch mx-4">
        <input
          type="text"
          placeholder="Searching for a unit? Click here!"
          class="w-full rounded-3xl p-2 text-center border border-black"
        />
      </div>
      <div class="grid grid-cols-auto-fill gap-4" style={{ "grid-template-columns": 'repeat(auto-fill, minmax(6rem, 1fr))' }}>
        <AddNewUnit onAdd={addUnit} />
        <For each={units()}>
          {(unit) => (
            <UnitItem
              unit={unit}
              onDelete={removeUnit}
            />
          )}
        </For>
      </div>
    </div>
  );
};

export default Units;
