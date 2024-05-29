import { For, Component, Show, onMount, createSignal } from 'solid-js';
import AddNewUnit from '../components/AddNewUnit';
import UnitItem from '../components/UnitItem';
import { useUnits } from '../UnitsProvider';
import { Unit } from '../interfaces';

const Units: Component = () => {
  const [searchInput, setSearchInput] = createSignal('');

  const { units, fetchUnits } = useUnits();
  const filteredUnits = (): Unit[] => {
    return units().filter(unit => unit.singular_name.includes(searchInput()) || unit.plural_name.includes(searchInput()));
  }
  onMount(() => {
    fetchUnits();
  });

  return (
    <div class="min-h-full p-4 bg-beige flex flex-col gap-4 flex-1 py-5 bg-mid-beige">
      <div class="">
          <h1 class="text-3xl text-center">Available units</h1>
      </div>
      <div class="flex justify-stretch items-stretch mx-4 col-span-4">
        <input
          type="text"
          placeholder="Searching for an unit? Click here!"
          onInput={(e) => setSearchInput(e.currentTarget.value.toLowerCase())}
          class="w-full rounded-3xl py-1 px-3 text-center border border-black max-w-96 mx-auto"
        />
      </div>
      <div class="grid grid-cols-auto-fill gap-4 max-auto" style={{ 'grid-template-columns': 'repeat(auto-fill, minmax(6rem, 1fr))' }}>
        <AddNewUnit />
        <Show when={units() === null || units === undefined}>
          <div>Loading units...</div>
        </Show>
        <For each={filteredUnits()}>
          {(unit) => (
            <UnitItem
              unit={unit}
              refetchUnits={fetchUnits}
            />
          )}
        </For>
      </div>
    </div>
  );
};

export default Units;
