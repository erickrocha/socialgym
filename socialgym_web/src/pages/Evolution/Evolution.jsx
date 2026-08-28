import React, { useEffect, useMemo, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { useTranslation } from 'react-i18next';
import { AppHeader, Sidebar } from '../../commons/gui/index.js';
import { addEvolutionCheckin, getEvolutionCheckins } from '../../redux/reducers/timeline/index.js';
import './Evolution.scss';
import axios from '../../axios.config.js';

const toRange = (period, customStart, customEnd) => {
    const now = new Date();
    const end = new Date(now);
    end.setHours(23, 59, 59, 999);

    const start = new Date(now);
    if (period === 'week') start.setDate(now.getDate() - 7);
    if (period === 'month') start.setMonth(now.getMonth() - 1);
    if (period === 'half') start.setMonth(now.getMonth() - 6);

    if (period === 'custom') {
        return {
            startDate: `${customStart}T00:00:00`,
            endDate: `${customEnd}T23:59:59`,
        };
    }

    return {
        startDate: start.toISOString().slice(0, 19),
        endDate: end.toISOString().slice(0, 19),
    };
};

const emptyForm = {
    note: '',
    weight: '',
    bodyFatPct: '',
    muscleMassPct: '',
    chest: '',
    waist: '',
    hip: '',
};

const Evolution = () => {
    const { t } = useTranslation('common');
    const dispatch = useDispatch();
    const { person } = useSelector((state) => state.person);
    const { checkins, checkinsLoading, checkinsSubmitting, checkinsError } = useSelector((state) => state.timeline);
    const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);
    const [error, setError] = useState(checkinsError);
    const [period, setPeriod] = useState('week');
    const [form, setForm] = useState(emptyForm);
    const [healthConsent, setHealthConsent] = useState(false);
    const [healthConsentLoading, setHealthConsentLoading] = useState(true);

    const [customStart, setCustomStart] = useState(new Date(Date.now() - 7 * 86400000).toISOString().slice(0, 10));
    const [customEnd, setCustomEnd] = useState(new Date().toISOString().slice(0, 10));

    const load = async () => {
        const range = toRange(period, customStart, customEnd);
        await dispatch(getEvolutionCheckins(range));
    };

    useEffect(() => {
        load();
        axios.get('/workout/api/people/me/consents')
            .then(({ data }) => setHealthConsent(data.some((item) => item.document === 'health_data' && !item.revokedAt)))
            .finally(() => setHealthConsentLoading(false));
    }, [period]);

    const acceptHealthConsent = async () => {
        setHealthConsentLoading(true);
        try {
            const { data: legal } = await axios.get('/legal/documents/health_data');
            await axios.post('/workout/api/people/me/consents', {
                document: 'health_data', version: legal.version, accepted: true,
            });
            setHealthConsent(true);
        } catch (err) {
            setError(err?.response?.data?.message || 'Não foi possível registrar o consentimento.');
        } finally {
            setHealthConsentLoading(false);
        }
    };

    useEffect(() => {
        setError(checkinsError);
    }, [checkinsError]);

    const latest = useMemo(() => checkins[0] || null, [checkins]);

    const submitCheckin = async (e) => {
        e.preventDefault();
        if (!person?.uuid) {
            setError(t('evolution.errors.userNotLoaded'));
            return;
        }
        setError(null);

        try {
            const payload = {
                personUuid: person.uuid,
                note: form.note,
                visibility: 'Private',
                composition: {
                    weight: form.weight ? Number(form.weight) : null,
                    bodyFatPct: form.bodyFatPct ? Number(form.bodyFatPct) : null,
                    muscleMassPct: form.muscleMassPct ? Number(form.muscleMassPct) : null,
                },
                circumferences: {
                    chest: form.chest ? Number(form.chest) : null,
                    waist: form.waist ? Number(form.waist) : null,
                    hip: form.hip ? Number(form.hip) : null,
                },
            };

            await dispatch(addEvolutionCheckin(payload)).unwrap();
            setForm(emptyForm);
        } catch (err) {
            setError(err?.message || t('evolution.errors.save'));
        }
    };

    return (
        <div className="evolution-page">
            <AppHeader person={person} />
            <div className="evolution-page__layout">
                <Sidebar person={person} isCollapsed={isSidebarCollapsed} onToggle={() => setIsSidebarCollapsed((prev) => !prev)} />
                <main className="evolution-page__content">
                    <header className="evolution-page__header">
                        <h1>{t('evolution.title')}</h1>
                        <div className="evolution-page__filters">
                            <button className={period === 'week' ? 'active' : ''} onClick={() => setPeriod('week')}>{t('evolution.filters.week')}</button>
                            <button className={period === 'month' ? 'active' : ''} onClick={() => setPeriod('month')}>{t('evolution.filters.month')}</button>
                            <button className={period === 'half' ? 'active' : ''} onClick={() => setPeriod('half')}>{t('evolution.filters.half')}</button>
                            <button className={period === 'custom' ? 'active' : ''} onClick={() => setPeriod('custom')}>{t('evolution.filters.custom')}</button>
                        </div>
                        {period === 'custom' && (
                            <div className="evolution-page__custom-range">
                                <input type="date" value={customStart} onChange={(e) => setCustomStart(e.target.value)} />
                                <input type="date" value={customEnd} onChange={(e) => setCustomEnd(e.target.value)} />
                                <button onClick={load}>{t('evolution.apply')}</button>
                            </div>
                        )}
                    </header>

                    {latest && (
                        <section className="evolution-latest">
                            <article>
                                <h2>{latest?.composition?.weight ?? '-'} kg</h2>
                                <p>{t('evolution.metrics.weight')}</p>
                            </article>
                            <article>
                                <h2>{latest?.composition?.bodyFatPct ?? '-'} %</h2>
                                <p>{t('evolution.metrics.bodyFat')}</p>
                            </article>
                            <article>
                                <h2>{latest?.circumferences?.waist ?? '-'} cm</h2>
                                <p>{t('evolution.metrics.waist')}</p>
                            </article>
                        </section>
                    )}

                    {!healthConsentLoading && !healthConsent && (
                        <section className="evolution-form" aria-labelledby="health-consent-title">
                            <h2 id="health-consent-title">Dados opcionais de saúde</h2>
                            <p>Medidas corporais são opcionais e usadas somente para seu acompanhamento de bem-estar. Não constituem diagnóstico.</p>
                            <p><a href="/health-consent" target="_blank" rel="noreferrer">Leia o consentimento destacado</a>.</p>
                            <button type="button" onClick={acceptHealthConsent}>Concordar e habilitar medições</button>
                        </section>
                    )}

                    {healthConsent && <form className="evolution-form" onSubmit={submitCheckin}>
                        <h3>{t('evolution.newCheckin')}</h3>
                        <div className="evolution-form__grid">
                            <input placeholder={t('evolution.form.weight')} value={form.weight} onChange={(e) => setForm((prev) => ({ ...prev, weight: e.target.value }))} />
                            <input placeholder={t('evolution.form.bodyFat')} value={form.bodyFatPct} onChange={(e) => setForm((prev) => ({ ...prev, bodyFatPct: e.target.value }))} />
                            <input placeholder={t('evolution.form.muscleMass')} value={form.muscleMassPct} onChange={(e) => setForm((prev) => ({ ...prev, muscleMassPct: e.target.value }))} />
                            <input placeholder={t('evolution.form.chest')} value={form.chest} onChange={(e) => setForm((prev) => ({ ...prev, chest: e.target.value }))} />
                            <input placeholder={t('evolution.form.waist')} value={form.waist} onChange={(e) => setForm((prev) => ({ ...prev, waist: e.target.value }))} />
                            <input placeholder={t('evolution.form.hip')} value={form.hip} onChange={(e) => setForm((prev) => ({ ...prev, hip: e.target.value }))} />
                        </div>
                        <textarea placeholder={t('evolution.form.note')} value={form.note} onChange={(e) => setForm((prev) => ({ ...prev, note: e.target.value }))} />
                        <button type="submit" disabled={checkinsSubmitting}>{checkinsSubmitting ? t('evolution.saving') : t('evolution.save')}</button>
                        <p className="field-hint">Registros de bem-estar informados por você; não são diagnóstico ou orientação médica.</p>
                    </form>}

                    {checkinsLoading && <p className="evolution-state">{t('evolution.loading')}</p>}
                    {error && <p className="evolution-state evolution-state--error">{error}</p>}

                    {!checkinsLoading && !error && (
                        <section className="evolution-list">
                            {checkins.map((checkin) => (
                                <article key={checkin.uuid} className="checkin-row">
                                    <div>
                                        <strong>{new Date(checkin.createdAt || Date.now()).toLocaleString()}</strong>
                                        <p>{checkin.note || t('evolution.noNotes')}</p>
                                    </div>
                                    <div>
                                        <span>{t('evolution.metrics.weight')}: {checkin?.composition?.weight ?? '-'} kg</span>
                                        <span>{t('evolution.metrics.bodyFat')}: {checkin?.composition?.bodyFatPct ?? '-'} %</span>
                                        <span>{t('evolution.metrics.waist')}: {checkin?.circumferences?.waist ?? '-'} cm</span>
                                    </div>
                                </article>
                            ))}
                        </section>
                    )}
                </main>
            </div>
        </div>
    );
};

export default Evolution;
