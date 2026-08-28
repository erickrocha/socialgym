import { useEffect, useState } from 'react';
import axios from '../../axios.config.js';
import './Moderation.scss';

export default function Moderation() {
    const [reports, setReports] = useState([]);
    const [status, setStatus] = useState('open');
    const [error, setError] = useState('');
    const [loading, setLoading] = useState(true);

    const load = () => {
        setLoading(true);
        axios.get('/timeline/api/moderation/reports', { params: { status } })
            .then(({ data }) => { setReports(data); setError(''); })
            .catch((err) => setError(err?.response?.status === 403 ? 'Acesso exclusivo para moderadores.' : 'Não foi possível carregar a fila.'))
            .finally(() => setLoading(false));
    };
    useEffect(load, [status]);

    const decide = async (report, decision) => {
        const reason = window.prompt('Registre o motivo da decisão:');
        if (!reason) return;
        try {
            await axios.post(`/timeline/api/moderation/reports/${report.uuid}/decision`, { decision, reason });
            load();
        } catch (err) { setError(err?.response?.data?.message || 'Não foi possível registrar a decisão.'); }
    };

    return <main className="moderation-page">
        <header><h1>Moderação de conteúdo</h1><p>Decisões e motivos são registrados permanentemente.</p></header>
        <label>Estado da fila <select value={status} onChange={(e) => setStatus(e.target.value)}><option value="open">Aberta</option><option value="resolved">Resolvida</option></select></label>
        {error && <p role="alert">{error}</p>}
        {loading ? <p>Carregando…</p> : <ol className="moderation-list">
            {reports.map((report) => <li key={report.uuid} className={report.priority === 'urgent' ? 'urgent' : ''}>
                <h2>{report.reason} · {report.targetType}</h2>
                <p>{report.details || 'Sem detalhes adicionais.'}</p>
                <dl><dt>Publicação</dt><dd>{report.postId}</dd><dt>Alvo</dt><dd>{report.targetId}</dd><dt>Prioridade</dt><dd>{report.priority}</dd></dl>
                {report.status !== 'resolved' && <div><button type="button" onClick={() => decide(report, 'dismissed')}>Não viola</button><button type="button" className="remove" onClick={() => decide(report, 'removed')}>Remover conteúdo</button></div>}
            </li>)}
        </ol>}
    </main>;
}
