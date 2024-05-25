import { createSignal, For, Show, onMount } from 'solid-js';
import { styled } from 'solid-styled-components';
import { useIngredients } from '../IngredientsProvider';

const NewRecipePageContainer = styled.div`
  padding: 1rem;
  background-color: var(--beige);
  display: flex;
  flex-direction: column;
  gap: 1rem;
  flex: 1;
`;

const Header = styled.h1`
  text-align: center;
  font-size: 2.5rem;
`;

const InputGroup = styled.div`
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 1rem;
`;

const Label = styled.label`
  font-size: 1.2rem;
  font-weight: bold;
  color: black;
`;

const Input = styled.input`
  padding: 0.5rem;
  border-radius: 1rem;
  border: 2px solid var(--red);
  background-color: #fff5ee;
  color: black;
  &:focus {
    outline: none;
    border-color: #ff6f61;
  }
`;

const Textarea = styled.textarea`
  padding: 0.5rem;
  border-radius: 1rem;
  border: 2px solid var(--red);
  background-color: #fff5ee;
  color: black;
  resize: vertical;
  &:focus {
    outline: none;
    border-color: #ff6f61;
  }
`;

const Button = styled.button`
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 1rem;
  background-color: var(--red);
  color: #fff;
  font-size: 1rem;
  cursor: pointer;
  transition: background-color 0.3s ease;
  &:hover {
    background-color: #ff6f61;
  }

  @media (max-width: 768px) {
    padding: 0.5rem;
    font-size: 0.9rem;
  }

  @media (max-width: 480px) {
    padding: 0.5rem;
    font-size: 0.8rem;
  }
`;

const IngredientContainer = styled.div`
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
  flex-wrap: wrap;
`;

const StepContainer = styled.div`
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
`;

const Suggestions = styled.div`
  position: relative;
  margin-top: -0.5rem;
  margin-bottom: 0.5rem;
`;

const SuggestionList = styled.ul`
  position: absolute;
  z-index: 1;
  background-color: white;
  border: 1px solid #ccc;
  border-radius: 0.5rem;
  width: 100%;
  list-style: none;
  padding: 0;
  margin: 0;
`;

const SuggestionItem = styled.li`
  padding: 0.5rem;
  cursor: pointer;
  &:hover {
    background-color: var(--beige);
  }
`;

const availableUnits = [
  { id: 1, name: "g" },
  { id: 2, name: "kg" },
  { id: 3, name: "ml" },
  { id: 4, name: "l" },
  { id: 5, name: "cup" },
  { id: 6, name: "tsp" },
  { id: 7, name: "tbsp" }
];

export default function NewRecipePage() {
  const { ingredients, fetchIngredients } = useIngredients();

  onMount(() => {
    fetchIngredients();
    setIngredientSuggestions(ingredients());
  });

  const [name, setName] = createSignal("");
  const [description, setDescription] = createSignal("");
  const [recipe_ingredients, setRecipeIngredients] = createSignal([{ ingredient: "", ingredientId: null, unit: "", unitId: null, quantity: "" }]);
  const [steps, setSteps] = createSignal([{ instruction: "", number: 1 }]);

  const [ingredientSuggestions, setIngredientSuggestions] = createSignal(ingredients());
  const [unitSuggestions, setUnitSuggestions] = createSignal([]);
  const [showIngredientSuggestions, setShowIngredientSuggestions] = createSignal(false);
  const [showUnitSuggestions, setShowUnitSuggestions] = createSignal(false);

  const addIngredientField = () => {
    setRecipeIngredients([...recipe_ingredients(), { ingredient: "", ingredientId: null, unit: "", unitId: null, quantity: "" }]);
  };

  const addStepField = () => {
    setSteps([...steps(), { instruction: "", number: steps().length + 1 }]);
  };

  const handleIngredientChange = (index, field, value) => {
    const updatedIngredients = recipe_ingredients().map((ingredient, i) => i === index ? { ...ingredient, [field]: value } : ingredient);
    setRecipeIngredients(updatedIngredients);
    if (field === "ingredient") {
      setIngredientSuggestions(ingredients().filter(ing => ing.singular_name.toLowerCase().includes(value.toLowerCase())));
      setShowIngredientSuggestions(true);
    } else if (field === "unit") {
      setUnitSuggestions(availableUnits.filter(unit => unit.name.toLowerCase().includes(value.toLowerCase())));
      setShowUnitSuggestions(true);
    }
  };

  const handleIngredientBlur = (index) => {
    const ingredient = recipe_ingredients()[index].ingredient;
    const matchedIngredient = ingredients().find(ing => ing.singular_name.toLowerCase() === ingredient.toLowerCase());
    if (matchedIngredient) {
      const updatedIngredients = recipe_ingredients().map((ingredient, i) => i === index ? { ...ingredient, ingredientId: matchedIngredient.id } : ingredient);
      setRecipeIngredients(updatedIngredients);
    }
    setShowIngredientSuggestions(false);
  };

  const handleSuggestionClick = (index, field, value, idField, id) => {
    const updatedIngredients = recipe_ingredients().map((ingredient, i) => i === index ? { ...ingredient, [field]: value, [idField]: id } : ingredient);
    setRecipeIngredients(updatedIngredients);
    setShowIngredientSuggestions(false);
    setShowUnitSuggestions(false);
  };

  const handleStepChange = (index, value) => {
    const updatedSteps = steps().map((step, i) => i === index ? { ...step, instruction: value } : step);
    setSteps(updatedSteps);
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    // Handle the form submission logic
    console.log({
      name: name(),
      description: description(),
      ingredients: recipe_ingredients(),
      steps: steps()
    });
  };

  return (
    <NewRecipePageContainer>
      <Header>New Recipe</Header>
      <form onSubmit={handleSubmit}>
        <InputGroup>
          <Label for="name">Name</Label>
          <Input id="name" type="text" value={name()} onInput={(e) => setName(e.target.value)} />
        </InputGroup>
        <InputGroup>
          <Label for="description">Description</Label>
          <Textarea id="description" value={description()} onInput={(e) => setDescription(e.target.value)} rows="4" />
        </InputGroup>
        <InputGroup>
          <Label>Ingredients</Label>
          <For each={recipe_ingredients()}>
            {(ingredient, index) => (
              <IngredientContainer>
                <Input
                  type="text"
                  placeholder="Ingredient"
                  value={ingredient.ingredient}
                  onInput={(e) => handleIngredientChange(index, "ingredient", e.target.value)}
                  onFocus={() => setShowIngredientSuggestions(true)}
                  onBlur={() => handleIngredientBlur(index())} // Directly call the function
                  data-ingredient-id={ingredient.ingredientId}
                />
                <Suggestions>
                  <Show when={showIngredientSuggestions() && ingredientSuggestions().length > 0}>
                    <SuggestionList>
                      <For each={ingredientSuggestions()}>
                        {(suggestion) => (
                          <SuggestionItem onClick={() => handleSuggestionClick(index, "ingredient", suggestion.singular_name, "ingredientId", suggestion.id)}>
                            {suggestion.singular_name}
                          </SuggestionItem>
                        )}
                      </For>
                    </SuggestionList>
                  </Show>
                </Suggestions>
                <Input
                  type="text"
                  placeholder="Unit"
                  value={ingredient.unit}
                  onInput={(e) => handleIngredientChange(index, "unit", e.target.value)}
                  onFocus={() => setShowUnitSuggestions(true)}
                  onBlur={() => setShowUnitSuggestions(false)} // Directly call the function
                />
                <Suggestions>
                  <Show when={showUnitSuggestions() && unitSuggestions().length > 0}>
                    <SuggestionList>
                      <For each={unitSuggestions()}>
                        {(suggestion) => (
                          <SuggestionItem onClick={() => handleSuggestionClick(index, "unit", suggestion.name, "unitId", suggestion.id)}>
                            {suggestion.name}
                          </SuggestionItem>
                        )}
                      </For>
                    </SuggestionList>
                  </Show>
                </Suggestions>
                <Input
                  type="number"
                  placeholder="Quantity"
                  value={ingredient.quantity}
                  onInput={(e) => handleIngredientChange(index, "quantity", e.target.value)}
                />
              </IngredientContainer>
            )}
          </For>
          <Button type="button" onClick={addIngredientField}>Add Ingredient</Button>
        </InputGroup>
        <InputGroup>
          <Label>Steps</Label>
          <For each={steps()}>
            {(step, index) => (
              <StepContainer>
                <Label>Step {index() + 1}</Label>
                <Textarea
                  value={step.instruction}
                  onInput={(e) => handleStepChange(index(), e.target.value)}
                  rows="2"
                />
              </StepContainer>
            )}
          </For>
          <Button type="button" onClick={addStepField}>Add Step</Button>
        </InputGroup>
        <Button type="submit">Save Recipe</Button>
      </form>
    </NewRecipePageContainer>
  );
}
