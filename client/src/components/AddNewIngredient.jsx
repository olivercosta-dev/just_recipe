import { styled } from 'solid-styled-components';
import addNewIcon from '../assets/add-new-icon.svg';
import { createSignal } from 'solid-js';
import baseUrl from '../baseUrl';
import { useIngredients } from '../IngredientsProvider';

const Dialog = styled('dialog')`
  border: none;
  padding: 1rem;
  border-radius: 20px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  background: white;
  width: 100%;
  max-width: 20rem;
  position: absolute;
  font-family: 'Arial', sans-serif;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  &::backdrop {
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(0, 0, 0, 0.3);
  }
`;

const ModalContent = styled('div')`
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const CloseButton = styled('button')`
  position: absolute;
  top: 10px;
  right: 10px;
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #ff6b6b;
`;

const FormLabel = styled('label')`
  display: block;
  margin-bottom: 0.5rem;
  font-size: 1rem;
  color: #333;
`;

const FormInput = styled('input')`
  width: 100%;
  padding: 0.5rem;
  box-sizing: border-box;
  border: 1px solid #ccc;
  border-radius: 10px;
  font-size: 1rem;
  margin-bottom: 1rem;
`;

const SubmitButton = styled('button')`
  padding: 0.5rem 1rem;
  background-color: #ff6b6b;
  color: white;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 1rem;
  font-family: 'Arial', sans-serif;
  margin-top: 1rem;

  &:hover {
    background-color: #e55a5a;
  }

  @media (max-width: 480px) {
    width: 100%;
  }

  &.success {
    background-color: #28a745; /* green */
    color: white;
  }

  &.error {
    background-color: #dc3545; /* red */
    color: white;
  }
`;

const AddNewButtonContainer = styled('div')`
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: center;
  background-color: #f9f9f9;
  border-radius: 1rem;
  padding: 0.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: scale 0.3s;
  &:hover{
    scale: 1.05;
    cursor: pointer;
  }
`;

const AddNewImage = styled('img')`
  min-height: 3rem;
  max-height: 4rem;
  aspect-ratio: 1/1;
`

const AddNewText = styled('span')`
  font-size: 1rem;
  text-align: center;
  color: #333;
 
`
function AddNewIngredient() {
  let dialogRef;

  const [singularName, setSingularName] = createSignal('');
  const [pluralName, setPluralName] = createSignal('');
  const [submitButtonText, setSubmitButtonText] = createSignal('Add Ingredient');
  const [submitButtonClass, setSubmitButtonClass] = createSignal('');
  const {fetchIngredients} = useIngredients();
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
      <AddNewButtonContainer
        onClick={showDialog}
      >
        <AddNewImage src={addNewIcon} alt="Add New Icon" />
        <AddNewText>Add new</AddNewText>
      </AddNewButtonContainer>
      <Dialog ref={(el) => (dialogRef = el)}>
        <ModalContent>
          <CloseButton onClick={closeDialog}>x</CloseButton>
          <h2>Add New Ingredient</h2>
          <form onSubmit={onSubmit}>
            <FormLabel for="singular-name">Singular Name</FormLabel>
            <FormInput
              type="text"
              id="singular-name"
              name="singular-name"
              required
              onChange={(e) => setSingularName(e.currentTarget.value)}
            />
            <FormLabel for="plural-name">Plural Name</FormLabel>
            <FormInput
              type="text"
              id="plural-name"
              name="plural-name"
              required
              onChange={(e) => setPluralName(e.currentTarget.value)}
            />
            <FormLabel for="ingredient-image">Attach Image</FormLabel>
            <FormInput
              type="file"
              id="ingredient-image"
              name="ingredient-image"
              accept="image/*"
            />
            <SubmitButton type="submit" class={submitButtonClass()}>
              {submitButtonText()}
            </SubmitButton>
          </form>
        </ModalContent>
      </Dialog>
    </>
  );
}

export default AddNewIngredient;
