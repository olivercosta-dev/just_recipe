import logo from '../assets/logo.svg'
import menuIcon from '../assets/settings.svg'
import { styled } from 'solid-styled-components';
import { createSignal } from 'solid-js';
const NavbarContainer = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 2px solid black;
  padding: 1rem;
`;

const LogoAndText = styled.div`
  display: flex;
  justify-content: start;
  align-items: center;
  gap: 0.2rem;
`;

const NavLogo = styled.img`
  min-height: 3rem;
`;

const NavLogoText = styled.span`
  font-size: 1rem;
  font-weight: bold;
`;

const MenuIcon = styled.img`
  height: 30px;
  cursor: pointer;
`;

const DropdownMenu = styled.div`
  position: absolute;
  top: 60px;
  right: 20px;
  background-color: white;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  overflow: hidden;
  display: ${(props) => (props.isOpen ? 'block' : 'none')};
`;

const MenuItem = styled.a`
  display: block;
  padding: 0.5rem 1rem;
  text-decoration: none;
  color: #333;
  &:hover {
    background-color: var(--red);
    cursor: pointer;
  }
`;

export default function Navbar() {
  const [isOpen, setIsOpen] = createSignal(false);

  const toggleMenu = () => setIsOpen(!isOpen());

  return (
    <>
      <NavbarContainer>
        <LogoAndText>
          <NavLogo src={logo} alt="The JustRecipe Official Logo." />
          <a href="/">
            <NavLogoText>Just Recipe!</NavLogoText>
          </a>
        </LogoAndText>
        <MenuIcon src={menuIcon} onClick={toggleMenu} />
      </NavbarContainer>
      <DropdownMenu isOpen={isOpen()}>
        <MenuItem href="/ingredients">Ingredients</MenuItem>
        <MenuItem href="/units">Units</MenuItem>
        <MenuItem href="/settings">Settings</MenuItem>
      </DropdownMenu>
    </>
  );
}

