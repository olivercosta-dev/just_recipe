import logo from '../assets/logo.svg'
import settingsLogo from '../assets/settings.svg'
export default function Navbar() {
    // This is gonna be the Home Page for now.
    return (<>
        <div class="navbar">
            <div class="logo-and-text">
                <img class='nav-logo' src={logo} alt="The JustRecipe Official Logo." />
                <a href="/">
                    <span class="nav-logo-text">Just Recipe!</span>
                </a>
            </div>
            <div class='settings-logo'>
                <a href="/ingredients">
                    <img src={settingsLogo}/>
                </a>
            </div>
        </div>
    </>
    );
}

