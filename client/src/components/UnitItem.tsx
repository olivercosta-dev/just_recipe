import { createSignal, Component } from 'solid-js';
import carrotIcon from '../assets/ingredient_icons/carrot.svg';
import baseUrl from '../baseUrl';
import { Unit } from '../interfaces'; // Ensure this path is correct

interface UnitItemProps {
  unit: Unit;
  onDelete: (unitId: string) => void;
}

const UnitItem: Component<UnitItemProps> = ({ unit, onDelete }) => {
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
        onDelete(unit.unit_id ?? '0');
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
    <div class="flex flex-col items-center justify-center relative bg-gray-200 rounded-3xl p-2 shadow hover:opacity-50">
      <img src={carrotIcon} alt="" class="min-h-12 max-h-16 aspect-square" />
      <button
        onClick={handleDelete}
        class="absolute left-1/2 top-1/2 transform -translate-x-1/2 -translate-y-1/2 p-1 bg-red-500 text-white rounded-md font-bold text-sm cursor-pointer shadow transition-all hover:bg-red-400 hover:scale-110"
      >
        Delete
      </button>
      <span class="text-center text-base mt-2">{unit.singular_name}</span> {/* Assuming 'name' is the correct property */}
      {feedbackMessage() && (
        <span class={`text-center text-sm mt-2 ${feedbackMessage() === 'Unit deleted successfully' ? 'text-green-500' : 'text-red-500'}`}>
          {feedbackMessage()}
        </span>
      )}
    </div>
  );
};

export default UnitItem;
