import { useState } from 'react';
import './CookieBanner.scss';

const STORAGE_KEY = 'socialgym-cookie-consent-v1';

export default function CookieBanner() {
    const [visible, setVisible] = useState(() => !localStorage.getItem(STORAGE_KEY));
    if (!visible) return null;

    const save = (analytics) => {
        localStorage.setItem(STORAGE_KEY, JSON.stringify({ necessary: true, analytics, updatedAt: new Date().toISOString() }));
        window.dispatchEvent(new CustomEvent('socialgym-cookie-consent-changed', { detail: { analytics } }));
        setVisible(false);
    };

    return (
        <section className="cookie-banner" aria-label="Preferências de cookies" role="dialog" aria-live="polite">
            <p>Usamos armazenamento necessário para manter sua sessão. Métricas opcionais só serão ativadas com sua autorização.</p>
            <div>
                <button type="button" onClick={() => save(false)}>Recusar não essenciais</button>
                <button type="button" onClick={() => save(true)}>Aceitar selecionados</button>
            </div>
        </section>
    );
}
