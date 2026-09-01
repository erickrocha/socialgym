import React from 'react';
import { useSelector } from 'react-redux';
import { Navigate, useNavigate } from 'react-router';
import { useTranslation } from 'react-i18next';
import './CompletedSets.scss';

/**
 * Full-page, read-only view of every set completed so far in the current
 * workout execution, grouped by exercise. Reached by tapping the
 * "Completed Sets" box on the workout runner.
 */
const CompletedSets = () => {
    const { t } = useTranslation('common');
    const navigate = useNavigate();
    const { active, workout, executedSets } = useSelector((state) => state.workoutExecution);

    if (!active) {
        return <Navigate to="/workouts" replace />;
    }

    const unit = t('workout.weightUnit');
    const isCardio = (category) => String(category || '').toLowerCase() === 'cardio';
    const volumeOf = (sets) => sets.reduce((acc, s) => acc + (s.weight * s.reps), 0);

    // Group by exercise, preserving completion order.
    const groups = [];
    const byId = new Map();
    for (const set of executedSets) {
        let group = byId.get(set.exerciseId);
        if (!group) {
            group = {
                exerciseId: set.exerciseId,
                exerciseName: set.exerciseName,
                category: set.category,
                sets: []
            };
            byId.set(set.exerciseId, group);
            groups.push(group);
        }
        group.sets.push(set);
    }

    const totalVolume = executedSets.reduce(
        (acc, s) => (isCardio(s.category) ? acc : acc + s.weight * s.reps),
        0
    );

    const formatClock = (iso) => {
        if (!iso) return '';
        const d = new Date(iso);
        return `${d.getHours().toString().padStart(2, '0')}:${d.getMinutes().toString().padStart(2, '0')}`;
    };

    return (
        <div className="completed-sets">
            <header className="completed-sets__header">
                <button
                    className="completed-sets__back"
                    onClick={() => navigate(-1)}
                    aria-label={t('workoutExecution.close')}
                >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                        <path d="M19 12H5M12 19l-7-7 7-7" />
                    </svg>
                </button>
                <div className="completed-sets__heading">
                    <h1>{t('workoutExecution.completedSets')} ({executedSets.length})</h1>
                    <p className="completed-sets__subheader">{workout?.name}</p>
                </div>
                <div className="completed-sets__chips">
                    <span className="completed-sets__chip">
                        🔁 {executedSets.length} {t('workoutExecution.set')}
                    </span>
                    <span className="completed-sets__chip">
                        🏋️ {Math.round(totalVolume).toLocaleString()} {unit}
                    </span>
                </div>
            </header>

            <div className="completed-sets__body">
                {groups.map((group) => (
                    <section key={group.exerciseId} className="completed-sets__group">
                        <div className="completed-sets__group-header">
                            <span className="completed-sets__group-name">{group.exerciseName}</span>
                            <span className="completed-sets__group-total">
                                {group.sets.length} {t('workoutExecution.set')}
                                {!isCardio(group.category) &&
                                    ` · ${Math.round(volumeOf(group.sets)).toLocaleString()} ${unit}`}
                            </span>
                        </div>
                        <ul className="completed-sets__rows">
                            {group.sets.map((set, idx) => (
                                <li key={idx} className="completed-sets__row">
                                    <span className="completed-sets__row-check">✓</span>
                                    <span className="completed-sets__row-set">
                                        {t('workoutExecution.set')} {set.setNumber}
                                    </span>
                                    <span className="completed-sets__row-details">
                                        {set.weight}{unit} × {set.reps}
                                        {set.completedAt && ` · ${formatClock(set.completedAt)}`}
                                    </span>
                                </li>
                            ))}
                        </ul>
                    </section>
                ))}
            </div>
        </div>
    );
};

export default CompletedSets;
