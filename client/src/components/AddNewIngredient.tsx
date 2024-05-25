import addNewIcon from '../assets/add-new-icon.svg';
import { createSignal, Component } from 'solid-js';
import baseUrl from '../baseUrl';

interface AddNewIngredientProps {
  onAdd: (ingredient: { singular_name: string; plural_name: string }) => void;
}

const AddNewIngredient: Component<AddNewIngredientProps> = (props) => {
  let dialogRef: HTMLDialogElement | undefined;

  const [singularName, setSingularName] = createSignal('');
  const [pluralName, setPluralName] = createSignal('');
  const [submitButtonText, setSubmitButtonText] = createSignal('Add Ingredient');
  const [submitButtonClass, setSubmitButtonClass] = createSignal('');
  const { fetchIngredients } = useIngredients();
  const showDialog = () => {
    dialogRef.showModal();
  };

  const closeDialog = () => {
    dialogRef.close();
  };

  const onSubmit = async (event) => {
    event.preventDefault();
    const formData = {
      singular_name: singularName(),
      plural_name: pluralName(),
    };
    try {
      const response = await fetch(`${baseUrl}/ingredients`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(formData),
      });

      if (response.ok) {
        setSubmitButtonText('Added Successfully');
        setSubmitButtonClass('success');
        fetchIngredients();
        closeDialog();
      } else {
        setSubmitButtonText('Failed to Add');
        setSubmitButtonClass('error');
      }
    } catch (error) {
      setSubmitButtonText('Failed to Add');
      setSubmitButtonClass('error');
    }
    fetchIngredients();
    // Reset button text and class after a delay
    setTimeout(() => {
      setSubmitButtonText('Add Ingredient');
      setSubmitButtonClass('');
    }, 1000);
  };

  return (
    <>
      <div
        class="flex flex-col items-stretch justify-center bg-gray-200 rounded-3xl p-2 shadow hover:scale-105 transition-transform cursor-pointer"
        onClick={showDialog}
      >
        <img src={addNewIcon} alt="Add New Icon" class="min-h-12 max-h-16 aspect-square" />
        <span class="text-center text-gray-800">Add new</span>
      </div>
      <dialog
        ref={(el) => (dialogRef = el)}
        class="border-none p-4 rounded-xl shadow-lg bg-white w-full max-w-md absolute font-sans left-1/2 top-1/2 transform -translate-x-1/2 -translate-y-1/2"
      >
        <div class="flex flex-col items-center">
          <button
            onClick={closeDialog}
            class="absolute top-2 right-2 bg-none border-none text-2xl text-red-500 cursor-pointer"
          >
            &times;
          </button>
          <h2>Add New Ingredient</h2>
          <form onSubmit={onSubmit} class="w-full">
            <label for="singular-name" class="block mb-2 text-sm font-medium text-gray-700">Singular Name</label>
            <input
              type="text"
              id="singular-name"
              name="singular-name"
              required
              onChange={(e) => setSingularName(e.currentTarget.value)}
              class="w-full p-2 box-border border border-gray-300 rounded-3xl text-base mb-4"
            />
            <label for="plural-name" class="block mb-2 text-sm font-medium text-gray-700">Plural Name</label>
            <input
              type="text"
              id="plural-name"
              name="plural-name"
              required
              onChange={(e) => setPluralName(e.currentTarget.value)}
              class="w-full p-2 box-border border border-gray-300 rounded-3xl text-base mb-4"
            />
            <label for="ingredient-image" class="block mb-2 text-sm font-medium text-gray-700">Attach Image</label>
            <input
              type="file"
              id="ingredient-image"
              name="ingredient-image"
              accept="image/*"
              class="w-full p-2 box-border border border-gray-300 rounded-3xl text-base mb-4"
            />
            <button
              type="submit"
              class={`p-2 bg-red-500 text-white rounded-3xl text-base mt-4 w-full ${submitButtonClass() === 'success' ? 'bg-green-500' : ''
                } ${submitButtonClass() === 'error' ? 'bg-red-700' : ''} hover:bg-red-400`}
            >
              {submitButtonText()}
            </button>
          </form>
        </div>
      </dialog>
    </>
  );
};

export default AddNewIngredient;
