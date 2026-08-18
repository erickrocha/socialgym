import React, { useEffect, useMemo, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { useTranslation } from 'react-i18next';
import { AppHeader, Sidebar } from '../../commons/gui/index.js';
import { getWorkoutSessions } from '../../redux/reducers/timeline/index.js';
import './WorkoutSessions.scss';

const toRange = (period, customStart, customEnd) => {
    const now = new Date();
    const end = new Date(now);
    end.setHours(23, 59, 59, 999);

    const start = new Date(now);
    if (period === 'week') start.setDate(now.getDate() - 7);
    if (period === 'month') start.setMonth(now.getMonth() - 1);
    if (period === 'quarter') start.setMonth(now.getMonth() - 3);

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

const WorkoutSessions = () => {
    const { t } = useTranslation('common');
    const dispatch = useDispatch();
    const { person } = useSelector((state) => state.person);
    const { sessions, sessionsLoading, sessionsError } = useSelector((state) => state.timeline);
    const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);
    const [period, setPeriod] = useState('week');

    const [customStart, setCustomStart] = useState(new Date(Date.now() - 7 * 86400000).toISOString().slice(0, 10));
    const [customEnd, setCustomEnd] = useState(new Date().toISOString().slice(0, 10));

    const load = async () => {
        const range = toRange(period, customStart, customEnd);
        await dispatch(getWorkoutSessions(range));
    };

    useEffect(() => {
        load();
    }, [period]);

    const summary = useMemo(() => {
        const count = sessions.length;
        const totalVolume = sessions.reduce((acc, item) => acc + Number(item.totalVolume || 0), 0);
        const totalDuration = sessions.reduce((acc, item) => acc + Number(item.duration || 0), 0);
        const avgDuration = count ? Math.round(totalDuration / count) : 0;

        return { count, totalVolume, avgDuration };
    }, [sessions]);

    const chartData = useMemo(() => {
        const list = sessions.slice(-10);
        const max = Math.max(...list.map((x) => Number(x.totalVolume || 0)), 1);
        return list.map((session) => ({
            label: new Date(session.completedAt || session.startedAt || Date.now()).toLocaleDateString(),
            value: Number(session.totalVolume || 0),
            pct: Math.max(8, Math.round((Number(session.totalVolume || 0) / max) * 100)),
        }));
    }, [sessions]);

    return (
        <div className="sessions-page">
            <AppHeader person={person} />
            <div className="sessions-page__layout">
                <Sidebar person={person} isCollapsed={isSidebarCollapsed} onToggle={() => setIsSidebarCollapsed((prev) => !prev)} />
                <main className="sessions-page__content">
                    <header className="sessions-page__header">
                        <h1>{t('workoutSessions.title')}</h1>
                        <div className="sessions-page__filters">
                            <button className={period === 'week' ? 'active' : ''} onClick={() => setPeriod('week')}>{t('workoutSessions.filters.week')}</button>
                            <button className={period === 'month' ? 'active' : ''} onClick={() => setPeriod('month')}>{t('workoutSessions.filters.month')}</button>
                            <button className={period === 'quarter' ? 'active' : ''} onClick={() => setPeriod('quarter')}>{t('workoutSessions.filters.quarter')}</button>
                            <button className={period === 'custom' ? 'active' : ''} onClick={() => setPeriod('custom')}>{t('workoutSessions.filters.custom')}</button>
                        </div>
                        {period === 'custom' && (
                            <div className="sessions-page__custom-range">
                                <input type="date" value={customStart} onChange={(e) => setCustomStart(e.target.value)} />
                                <input type="date" value={customEnd} onChange={(e) => setCustomEnd(e.target.value)} />
                                <button onClick={load}>{t('workoutSessions.apply')}</button>
                            </div>
                        )}
                    </header>

                    <section className="sessions-summary">
                        <article>
                            <h2>{summary.count}</h2>
                            <p>{t('workoutSessions.summary.sessions')}</p>
                        </article>
                        <article>
                            <h2>{summary.totalVolume.toFixed(0)} kg</h2>
                            <p>{t('workoutSessions.summary.totalVolume')}</p>
                        </article>
                        <article>
                            <h2>{Math.floor(summary.avgDuration / 60)}:{String(summary.avgDuration % 60).padStart(2, '0')}</h2>
                            <p>{t('workoutSessions.summary.avgDuration')}</p>
                        </article>
                    </section>

                    {sessionsLoading && <p className="sessions-state">{t('workoutSessions.loading')}</p>}
                    {sessionsError && <p className="sessions-state sessions-state--error">{sessionsError}</p>}

                    {!sessionsLoading && !sessionsError && (
                        <>
                            <section className="sessions-chart">
                                {chartData.length === 0 && <p>{t('workoutSessions.empty')}</p>}
                                {chartData.map((bar) => (
                                    <div key={`${bar.label}-${bar.value}`} className="sessions-chart__item">
                                        <div className="sessions-chart__bar" style={{ height: `${bar.pct}%` }} title={`${bar.value} kg`} />
                                        <span>{bar.label}</span>
                                    </div>
                                ))}
                            </section>

                            <section className="sessions-list">
                                {sessions.map((session) => (
                                    <article key={session.uuid} className="session-row">
                                        <div>
                                            <h3>{session.workoutName}</h3>
                                            <span>{new Date(session.completedAt || session.startedAt || Date.now()).toLocaleString()}</span>
                                        </div>
                                        <div>
                                            <strong>{Number(session.totalVolume || 0).toFixed(0)} kg</strong>
                                            <span>{session.totalSets || 0} {t('workoutSessions.sets')}</span>
                                        </div>
                                    </article>
                                ))}
                            </section>
                        </>
                    )}
                </main>
            </div>
        </div>
    );
};

export default WorkoutSessions;
