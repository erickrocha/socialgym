import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useDispatch, useSelector } from 'react-redux';
import { AppHeader, Button, Toast } from '../../commons/gui';
import { fetchMySettings, updateMySettings } from '../../redux/reducers/settings/settings.actions';
import { setLocalSettings } from '../../redux/reducers/settings/settings.slice';
import './Settings.scss';
import axios from '../../axios.config.js';

export const Settings = () => {
    const { t, i18n } = useTranslation();
    const dispatch = useDispatch();
    const person = useSelector((state) => state.person?.person);
    const { settings, loading } = useSelector((state) => state.settings);

    const [language, setLanguage] = useState('pt-BR');
    const [theme, setTheme] = useState('dark');
    const [notificationsEnabled, setNotificationsEnabled] = useState(true);
    const [homePage, setHomePage] = useState('/home');
    const [toastMessage, setToastMessage] = useState('');
    const [exports, setExports] = useState([]);
    const [requestingExport, setRequestingExport] = useState(false);

    useEffect(() => {
        dispatch(fetchMySettings());
        axios.get('/workout/api/people/me/data-exports').then(({ data }) => setExports(data)).catch(() => {});
    }, [dispatch]);

    useEffect(() => {
        if (settings) {
            setLanguage(settings.language || 'pt-BR');
            setTheme(settings.theme || 'dark');
            setNotificationsEnabled(settings.notificationsEnabled ?? true);
            setHomePage(settings.homePage || '/home');
        }
    }, [settings]);

    const handleSave = async (e) => {
        e.preventDefault();
        const payload = {
            language,
            theme,
            notificationsEnabled,
            homePage
        };

        dispatch(setLocalSettings(payload));
        i18n.changeLanguage(language);

        try {
            await dispatch(updateMySettings(payload));
            setToastMessage(t('settings.savedSuccess', 'Configurações salvas com sucesso!'));
        } catch {
            setToastMessage(t('settings.savedSuccess', 'Configurações salvas localmente!'));
        }
    };

    const requestExport = async () => {
        setRequestingExport(true);
        try {
            const { data } = await axios.post('/workout/api/people/me/data-exports');
            setExports((current) => [data, ...current]);
            setToastMessage('Sua exportação foi solicitada. Esta página mostrará quando estiver pronta.');
        } catch {
            setToastMessage('Não foi possível solicitar a exportação.');
        } finally { setRequestingExport(false); }
    };

    const downloadExport = async (id) => {
        try {
            const { data } = await axios.get(`/workout/api/people/me/data-exports/${id}/download`);
            window.location.assign(data.url);
        } catch { setToastMessage('A exportação ainda não está disponível.'); }
    };

    const deleteAccount = async () => {
        if (!window.confirm('Sua conta será desativada agora e excluída após o prazo informado. Deseja continuar?')) return;
        try {
            const { data } = await axios.post('/workout/api/people/me/account/delete', { immediate: false });
            localStorage.removeItem('auth');
            window.alert(`Exclusão agendada para ${new Date(data.scheduledAt).toLocaleDateString()}.`);
            window.location.assign('/login');
        } catch { setToastMessage('Não foi possível solicitar a exclusão da conta.'); }
    };

    return (
        <div className="settings-page">
            <AppHeader person={person} />
            <main className="settings-container">
                <div className="settings-card">
                    <h1>⚙️ {t('settings.pageTitle', 'Configurações e Preferências')}</h1>
                    <p className="subtitle">{t('settings.pageSubtitle', 'Personalize sua experiência no SocialGym')}</p>

                    <form onSubmit={handleSave} className="settings-form">
                        <div className="form-section">
                            <h3>🌐 {t('settings.languageSection', 'Idioma do Aplicativo')}</h3>
                            <select
                                value={language}
                                onChange={(e) => setLanguage(e.target.value)}
                                className="gui-select-custom"
                            >
                                <option value="pt-BR">Português (Brasil)</option>
                                <option value="en">English (US)</option>
                                <option value="es">Español</option>
                                <option value="fr">Français</option>
                                <option value="nl">Nederlands</option>
                            </select>
                        </div>

                        <div className="form-section">
                            <h3>🎨 {t('settings.themeSection', 'Tema de Cores')}</h3>
                            <div className="radio-group">
                                <label className={`radio-tile ${theme === 'dark' ? 'selected' : ''}`}>
                                    <input
                                        type="radio"
                                        name="theme"
                                        value="dark"
                                        checked={theme === 'dark'}
                                        onChange={() => setTheme('dark')}
                                    />
                                    <span>🌙 {t('settings.darkTheme', 'Modo Escuro (Padrão)')}</span>
                                </label>
                                <label className={`radio-tile ${theme === 'light' ? 'selected' : ''}`}>
                                    <input
                                        type="radio"
                                        name="theme"
                                        value="light"
                                        checked={theme === 'light'}
                                        onChange={() => setTheme('light')}
                                    />
                                    <span>☀️ {t('settings.lightTheme', 'Modo Claro')}</span>
                                </label>
                            </div>
                        </div>

                        <div className="form-section">
                            <h3>🔔 {t('settings.notificationsSection', 'Notificações')}</h3>
                            <label className="toggle-label">
                                <input
                                    type="checkbox"
                                    checked={notificationsEnabled}
                                    onChange={(e) => setNotificationsEnabled(e.target.checked)}
                                />
                                <span>{t('settings.enableNotifications', 'Receber alertas e lembretes de treino')}</span>
                            </label>
                        </div>

                        <div className="form-section">
                            <h3>🏠 {t('settings.homeSection', 'Página Inicial Preferida')}</h3>
                            <select
                                value={homePage}
                                onChange={(e) => setHomePage(e.target.value)}
                                className="gui-select-custom"
                            >
                                <option value="/home">{t('header.homeTitle', 'Feed de Notícias (/home)')}</option>
                                <option value="/workouts">{t('header.workoutTitle', 'Biblioteca de Treinos (/workouts)')}</option>
                                <option value="/evolution">{t('header.evolutionTitle', 'Progresso de Evolução (/evolution)')}</option>
                            </select>
                        </div>

                        <div className="form-actions">
                            <Button type="submit" disabled={loading}>
                                {loading ? t('application.loading', 'Salvando...') : t('settings.saveBtn', 'Salvar Configurações')}
                            </Button>
                        </div>
                    </form>

                    <section className="form-section" aria-labelledby="privacy-rights-title">
                        <h2 id="privacy-rights-title">Privacidade e seus dados</h2>
                        <p><a href="/privacy">Política de Privacidade</a> · <a href="/privacy/dpo">Fale com o Encarregado</a></p>
                        <button type="button" onClick={() => {
                            localStorage.removeItem('socialgym-cookie-consent-v1');
                            window.location.reload();
                        }}>Revisar preferências de cookies</button>
                        <Button type="button" onClick={requestExport} disabled={requestingExport}>
                            {requestingExport ? 'Solicitando…' : 'Baixar meus dados'}
                        </Button>
                        {exports.length > 0 && <ul className="data-export-list">
                            {exports.map((item) => <li key={item.id}>
                                <span>Solicitação de {new Date(item.createdAt).toLocaleString()} — {item.status}</span>
                                {item.status === 'ready' && <button type="button" onClick={() => downloadExport(item.id)}>Baixar ZIP</button>}
                            </li>)}
                        </ul>}
                    </section>

                    <section className="form-section danger-zone" aria-labelledby="danger-zone-title">
                        <h2 id="danger-zone-title">Excluir conta</h2>
                        <p>A conta será desativada imediatamente. A exclusão permanente ocorrerá após o período de segurança configurado.</p>
                        <button type="button" className="danger-button" onClick={deleteAccount}>Solicitar exclusão da conta</button>
                    </section>
                </div>
            </main>

            {toastMessage && (
                <Toast
                    message={toastMessage}
                    onClose={() => setToastMessage('')}
                    duration={3000}
                />
            )}
        </div>
    );
};

export default Settings;
