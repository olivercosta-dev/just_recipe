import { createSignal, createContext, useContext, JSX } from 'solid-js';
import baseUrl from './baseUrl';
import { Unit } from './interfaces';

interface UnitsContextType {
  units: () => Unit[];
  fetchUnits: () => Promise<void>;
}

export const UnitsContext = createContext<UnitsContextType | undefined>(undefined);

export function UnitsProvider(props: { children: JSX.Element }) {
  const [units, setUnits] = createSignal<Unit[]>([]);

  const fetchUnits = async () => {
    try {
      const response = await fetch(`${baseUrl}/units/all`);
      const data: Unit[] = await response.json();
      setUnits(data);
    } catch (error) {
      console.error('Failed to fetch units:', error);
    }
  };

  return (
    <UnitsContext.Provider value={{ units, fetchUnits }}>
      {props.children}
    </UnitsContext.Provider>
  );
}

export function useUnits() {
  const context = useContext(UnitsContext);
  if (!context) {
    throw new Error('useUnits must be used within an UnitsProvider');
  }
  return context;
}
