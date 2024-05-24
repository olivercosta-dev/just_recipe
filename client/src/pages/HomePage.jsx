import exploreLogo from '../assets/explore-logo.svg'
import friendsLogo from '../assets/friends-logo.svg'
import myProfileLogo from '../assets/my-profile-logo.svg'
import newRecipeLogo from '../assets/new-recipe-logo.svg'
import savedLogo from '../assets/saved-logo.svg'
import { Router, Route } from "@solidjs/router";
import NotFound from './NotFound'
import Explore from './ExploreRecipe'

export default function HomePage() {
    // This is gonna be the Home Page for now.
    return (
        <div class="main-page-btns">
            <div class="left-side">
                <a class='main-page-btn explore-btn' href="/explore">
                    <img class='explore-logo' src={exploreLogo} alt="An icon representing a sunrise behind a mountain." />
                    <span>Explore</span>
                </a>
                <div class='main-page-btn friends-btn'>
                    <img class='friends-logo' src={friendsLogo} alt="An icon of two cherries next to each other." />
                    <span>Friends</span>
                </div>
            </div>
            <div class="right-side">
                <div class='main-page-btn my-profile-btn'>
                    <img class='my-profile-logo' src={myProfileLogo} alt="An icon representing a sunrise behind a mountain." />
                    <span>My Profile</span>
                </div>
                <a class='main-page-btn new-recipe-btn' href='/newRecipe'>
                    <img class='new-recipe-logo' src={newRecipeLogo} alt="An icon of two cherries next to each other." />
                    <span>New Recipe</span>
                </a>
                <div class='main-page-btn saved-btn'>
                    <img class='new-recipe-logo' src={savedLogo} alt="An icon of two cherries next to each other." />
                    <span>Saved</span>
                </div>
            </div>
        </div>
    );
}
