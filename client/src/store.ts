import { createSignal } from 'solid-js';
import { Ingredient } from './interfaces';
import { Unit } from './interfaces';

export const [ingredients, setIngredients] = createSignal<Ingredient[]>([]);
export const [units, setUnits] = createSignal<Unit[]>([]);
