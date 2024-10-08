import { createSignal, Component } from 'solid-js';
import carrotIcon from '../assets/ingredient_icons/carrot.svg';
import baseUrl from '../baseUrl';
import { Unit } from '../interfaces';
interface UnitItemProps {
  unit: Unit
  refetchUnits: () => void;
}

const UnitItem: Component<UnitItemProps> = ({ unit, refetchUnits }) => {
  const [feedbackMessage, setFeedbackMessage] = createSignal<string>('');

  const handleDelete = async () => {
    try {
      const dataToSend = {
        unit_id: unit.unit_id
      };
      const response = await fetch(`${baseUrl}/units`, {
        method: 'DELETE',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(dataToSend)
      });

      if (response.ok) {
        setFeedbackMessage('Unit deleted successfully');
        refetchUnits();
      } else {
        setFeedbackMessage('Failed to delete unit');
      }
    } catch (error) {
      setFeedbackMessage('Failed to delete unit');
    }

    // Clear feedback message after a few seconds
    setTimeout(() => setFeedbackMessage(''), 3000);
  };

  return (
    <div class="group 
    flex flex-col 
    items-stretch 
    justify-center 
    relative bg-gray-200 
    rounded-3xl p-2 shadow">
      <img src={carrotIcon} alt={unit.singular_name}
       class="min-h-12 max-h-16 aspect-square group-hover:opacity-50" />
      <button
        onClick={handleDelete}
        class="
        opacity-0 
        absolute left-1/2 top-1/2 transform 
        -translate-x-1/2 
        -translate-y-1/2 p-1
         bg-red-500 text-white
          rounded-md font-bold text-sm cursor-pointer shadow transition-all hover:scale-110
          full-opacity-when-parent-hovered"
      >
        Delete
      </button>
      <span class="text-center text-base mt-2">{unit.singular_name}</span>
      {feedbackMessage() && (
        <span class={`text-center text-sm mt-2 ${feedbackMessage() === 'Unit deleted successfully' ? 'text-green-500' : 'text-red-500'}`}>
          {feedbackMessage()}
        </span>
      )}
    </div>

  );
};

export default UnitItem;
