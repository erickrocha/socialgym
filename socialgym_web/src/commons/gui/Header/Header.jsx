import React from 'react';

const Header = ({ children, className = '' }) => (
  <header className={`header ${className}`} role="banner">
    <div className="language-switcher-wrapper">{children}</div>
  </header>
);

export default Header;

