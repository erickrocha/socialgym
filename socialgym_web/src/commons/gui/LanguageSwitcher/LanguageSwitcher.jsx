import {useTranslation} from 'react-i18next';
import "./LanguageSwitcher.scss";
import clsx from "clsx";
import PropTypes from "prop-types";

const LanguageSwitcher = ({className}) => {
    const {i18n} = useTranslation();
    const change = (lng) => i18n.changeLanguage(lng);

    return (
        <div
            className={clsx("LanguageSwitcher", className)}
            role="group"
            aria-label="Language selection"
        >
            <button
                onClick={() => change('en')}
                aria-pressed={i18n.language === 'en'}
                aria-label="English"
                title="Select English language"
            >
                EN
            </button>
            <button
                onClick={() => change('pt-BR')}
                aria-pressed={i18n.language === 'pt-BR'}
                aria-label="Português Brasileiro"
                title="Selecionar idioma Português Brasileiro"
            >
                PT
            </button>
            <button
                onClick={() => change('es')}
                aria-pressed={i18n.language === 'es'}
                aria-label="Español"
                title="Seleccionar idioma Español"
            >
                ES
            </button>
            <button
                onClick={() => change('nl')}
                aria-pressed={i18n.language === 'nl'}
                aria-label="Nederlands"
                title="Nederlandse taal selecteren"
            >
                NL
            </button>
            <button
                onClick={() => change('fr')}
                aria-pressed={i18n.language === 'fr'}
                aria-label="Français"
                title="Sélectionner la langue française"
            >
                FR
            </button>
        </div>
    );
};

LanguageSwitcher.propTypes = {
    className: PropTypes.string,
}

export default LanguageSwitcher;
