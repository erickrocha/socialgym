import React from 'react';
import './Footer.scss';
import LanguageSwitcher from '../LanguageSwitcher/LanguageSwitcher.jsx';

const Footer = ({ children, className = '', showLanguageSwitcher = true }) => (
  <footer className={`footer ${className}`} role="contentinfo">
    {showLanguageSwitcher && <LanguageSwitcher />}
    {children}
  </footer>
);

export default Footer;

