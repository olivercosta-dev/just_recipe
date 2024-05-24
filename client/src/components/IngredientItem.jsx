import { styled } from 'solid-styled-components';
import { createSignal, useContext } from 'solid-js';
import carrotIcon from '../assets/ingredient_icons/carrot.svg';
import baseUrl from '../baseUrl';
import { useIngredients } from '../IngredientsProvider';

const Container = styled('div')`
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: center;
  position: relative;
  background-color: #f9f9f9;
  border-radius: 1rem;
  padding: 0.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  &:hover > img {
    opacity: 50%;
  }
`;

const Image = styled('img')`
  min-height: 3rem;
  max-height: 4rem;
  aspect-ratio: 1/1;
`;

const Delete = styled('button')`
  position: absolute;
  display: none;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  padding: 0.3rem 0.6rem;
  background-color: #ff6b6b;
  color: white;
  border: none;
  border-radius: 5px;
  font-size: 0.9rem;
  font-weight: bold;
  cursor: pointer;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: background-color 0.3s, transform 0.3s;

  ${Container}:hover > & {
    display: block;
  }

  &:hover {
    background-color: #e55a5a;
    transform: translate(-50%, -50%) scale(1.1);
  }
`;

const IngredientName = styled('span')`
  font-size: 1rem;
  text-align: center;
`;

const FeedbackMessage = styled('span')`
  font-size: 0.9rem;
  text-align: center;
  margin-top: 0.5rem;
  color: ${props => (props.success ? 'green' : 'red')};
`;

export default function IngredientItem({ ingredient }) {
  const [feedbackMessage, setFeedbackMessage] = createSignal('');
  const { fetchIngredients } = useIngredients();
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

      if (!response.ok) {
        setFeedbackMessage('Failed to delete ingredient');
      }
    } catch (error) {
      setFeedbackMessage('Failed to delete ingredient');
    }
    fetchIngredients();
    // Clear feedback message after a few seconds
    setTimeout(() => setFeedbackMessage(''), 1000);
  };

  return (
    <Container>
      <Image src={carrotIcon} alt="" />
      <Delete onClick={handleDelete}>Delete</Delete>
      <IngredientName>{ingredient.singular_name}</IngredientName>
      {feedbackMessage() && (
        <FeedbackMessage success={feedbackMessage() === 'Ingredient deleted successfully'}>
          {feedbackMessage()}
        </FeedbackMessage>
      )}
    </Container>
  );
}
