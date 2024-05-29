import logo from '../assets/logo.svg';
import menuIcon from '../assets/settings.svg';
import { createSignal, Component } from 'solid-js';

const Navbar: Component = () => {
  const [isOpen, setIsOpen] = createSignal(false);

  const toggleMenu = () => setIsOpen(!isOpen());

  return (
    <>
      <div class="flex justify-between items-center border-b-2 border-black p-4">
        <div class="flex items-center gap-2">
          <img src={logo} alt="The JustRecipe Official Logo." class="min-h-12" />
          <a href="/" class="text-lg font-bold">Just Recipe!</a>
        </div>
        <img src={menuIcon} onClick={toggleMenu} class="h-8 cursor-pointer" />
      </div>
      <div
        class={`absolute top-16 right-5 bg-white shadow-lg rounded-2xl overflow-hidden ${
          isOpen() ? 'block' : 'hidden'
        }`}
      >
        <a href="/ingredients" class="block px-4 py-2 text-gray-800 hover:bg-red-500 hover:text-white cursor-pointer">
          Ingredients
        </a>
        <a href="/units" class="block px-4 py-2 text-gray-800 hover:bg-red-500 hover:text-white cursor-pointer">
          Units
        </a>
        <a href="/settings" class="block px-4 py-2 text-gray-800 hover:bg-red-500 hover:text-white cursor-pointer">
          Settings
        </a>
      </div>
    </>
  );
}

export default Navbar;
