import exploreLogo from '../assets/explore-logo.svg';
import friendsLogo from '../assets/friends-logo.svg';
import myProfileLogo from '../assets/my-profile-logo.svg';
import newRecipeLogo from '../assets/new-recipe-logo.svg';
import savedLogo from '../assets/saved-logo.svg';
import { Component } from 'solid-js';
import { Link } from '@kobalte/core';



const Home: Component = () => {
    return (
        <div class='flex justify-center gap-2 py-5'>
            <div class="flex justify-center gap-3 flex-col">
                <a href="/explore"
                   class="flex flex-col items-center justify-around border-2 p-4 decoration-none bg-mid-beige hover:cursor-pointer flex-1 rounded-3xl">
                    <img class='explore-logo max-h-[8rem] aspect-square' src={exploreLogo} alt="An icon representing a sunrise behind a mountain." />
                    <span>Explore</span>
                </a>
                <a class="flex flex-col items-center justify-around border-2 p-4 decoration-none hover:cursor-pointer flex-1 bg-moss-green rounded-3xl">
                    <img class='friends-logo max-h-[8rem] aspect-square' src={friendsLogo} alt="An icon of two cherries next to each other." />
                    <span>Friends</span>
                </a>
            </div>
            <div class='flex flex-col justify-between gap-3 min-h-full'>
                <a class='hover:cursor-pointer flex flex-col items-center justify-around border-2 p-4 decoration bg-dark-beige rounded-3xl'> 
                    <img class='my-profile-logo max-h-[8rem] ' src={myProfileLogo} alt="An icon representing a sunrise behind a mountain." />
                    <span>My Profile</span>
                </a>
                <a href="/newRecipe" class="flex flex-col items-center justify-around border-2 p-4 decoration bg-japanese-light-blue rounded-3xl">
                    <img class='new-recipe-logo max-h-[8rem] aspect-square' src={newRecipeLogo} alt="An icon of two cherries next to each other." />
                    <span>New Recipe</span>
                </a>
                <div class='flex flex-col items-center justify-around border-2 p-4 decoration bg-candy-pink hover:cursor-pointer rounded-3xl'>
                    <img class='new-recipe-logo max-h-[8rem] aspect-square' src={savedLogo} alt="An icon of two cherries next to each other." />
                    <span>Saved</span>
                </div>
            </div>
        </div>
    );
};

export default Home;
