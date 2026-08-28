import { useEffect, useState } from 'react';
import { Link, useParams } from 'react-router';
import axios from '../../axios.config';
import './LegalDocument.scss';

const ALLOWED_DOCUMENTS = new Set(['terms', 'privacy', 'health_data']);

export default function LegalDocument({ document: configuredDocument }) {
    const params = useParams();
    const document = configuredDocument || params.document;
    const [legal, setLegal] = useState(null);
    const [error, setError] = useState(false);

    useEffect(() => {
        if (!ALLOWED_DOCUMENTS.has(document)) {
            setError(true);
            return;
        }
        axios.get(`/legal/documents/${document}`)
            .then(({ data }) => setLegal(data))
            .catch(() => setError(true));
    }, [document]);

    return (
        <main className="legal-document">
            <Link to="/">← SocialGym</Link>
            {error && <p role="alert">Não foi possível carregar este documento.</p>}
            {!error && !legal && <p aria-live="polite">Carregando…</p>}
            {legal && (
                <article>
                    <p className="legal-document__version">Versão {legal.version}</p>
                    <div className="legal-document__content">{legal.content}</div>
                </article>
            )}
        </main>
    );
}
