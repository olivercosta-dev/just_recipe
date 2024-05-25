import { createSignal, Component } from 'solid-js';
import carrotIcon from '../assets/ingredient_icons/carrot.svg';
import baseUrl from '../baseUrl';
interface IngredientItemProps {
  ingredient: {
    ingredient_id: string;
    singular_name: string;
    // Add other fields as needed
  };
  onDelete: (ingredientId: string) => void;
}

const IngredientItem: Component<IngredientItemProps> = ({ ingredient, onDelete }) => {
  const [feedbackMessage, setFeedbackMessage] = createSignal<string>('');

  const handleDelete = async () => {
    try {
      const dataToSend = {
        ingredient_id: ingredient.ingredient_id
      };
      const response = await fetch(`${baseUrl}/ingredients`, {
        method: 'DELETE',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(dataToSend)
      });

      if (response.ok) {
        setFeedbackMessage('Ingredient deleted successfully');
        onDelete(ingredient.ingredient_id);
      } else {
        setFeedbackMessage('Failed to delete ingredient');
      }
    } catch (error) {
      setFeedbackMessage('Failed to delete ingredient');
    }

    // Clear feedback message after a few seconds
    setTimeout(() => setFeedbackMessage(''), 3000);
  };

  return (
    <div class="flex flex-col items-stretch justify-center relative bg-gray-200 rounded-3xl p-2 shadow hover:opacity-50">
      <img src={carrotIcon} alt={ingredient.singular_name} class="min-h-12 max-h-16 aspect-square" />
      <button
        onClick={handleDelete}
        class="absolute left-1/2 top-1/2 transform -translate-x-1/2 -translate-y-1/2 p-1 bg-red-500 text-white rounded-md font-bold text-sm cursor-pointer shadow transition-all hover:bg-red-400 hover:scale-110"
      >
        Delete
      </button>
      <span class="text-center text-base mt-2">{ingredient.singular_name}</span>
      {feedbackMessage() && (
        <span class={`text-center text-sm mt-2 ${feedbackMessage() === 'Ingredient deleted successfully' ? 'text-green-500' : 'text-red-500'}`}>
          {feedbackMessage()}
        </span>
      )}
    </div>
  );
};

export default IngredientItem;
