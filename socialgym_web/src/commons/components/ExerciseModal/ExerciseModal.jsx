import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Modal, TextField, Button, Select, VisibilityDropdown } from '../../gui';
import './ExerciseModal.scss';

export const ExerciseModal = ({ isOpen, onClose, onSubmit, initialData = null, loading = false }) => {
    const { t } = useTranslation();
    const [name, setName] = useState('');
    const [description, setDescription] = useState('');
    const [category, setCategory] = useState('Chest');
    const [sets, setSets] = useState(3);
    const [repsOrDuration, setRepsOrDuration] = useState(10);
    const [visibility, setVisibility] = useState('Public');

    useEffect(() => {
        if (initialData) {
            setName(initialData.name || '');
            setDescription(initialData.description || '');
            setCategory(initialData.category || 'Chest');
            setSets(initialData.sets || 3);
            setRepsOrDuration(initialData.reps_or_duration || initialData.repsOrDuration || 10);
            setVisibility(initialData.visibility || 'Public');
        } else {
            setName('');
            setDescription('');
            setCategory('Chest');
            setSets(3);
            setRepsOrDuration(10);
            setVisibility('Public');
        }
    }, [initialData, isOpen]);

    const handleSubmit = (e) => {
        e.preventDefault();
        if (!name.trim()) return;
        onSubmit({
            name,
            description,
            category,
            sets: Number(sets),
            reps_or_duration: Number(repsOrDuration),
            visibility
        });
    };

    const categories = [
        { label: t('exercises.categories.chest', 'Peito'), value: 'Chest' },
        { label: t('exercises.categories.back', 'Costas'), value: 'Back' },
        { label: t('exercises.categories.legs', 'Pernas'), value: 'Legs' },
        { label: t('exercises.categories.shoulders', 'Ombros'), value: 'Shoulders' },
        { label: t('exercises.categories.arms', 'Braços'), value: 'Arms' },
        { label: t('exercises.categories.core', 'Abdômen'), value: 'Core' },
        { label: t('exercises.categories.cardio', 'Cardio'), value: 'Cardio' },
        { label: t('exercises.categories.other', 'Outro'), value: 'Other' },
    ];

    if (!isOpen) return null;

    return (
        <Modal isOpen={isOpen} onClose={onClose} title={initialData ? t('exercises.editModalTitle', 'Editar Exercício') : t('exercises.createModalTitle', 'Criar Exercício')}>
            <form onSubmit={handleSubmit} className="exercise-modal-form">
                <TextField
                    label={t('exercises.nameLabel', 'Nome do Exercício')}
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                    placeholder={t('exercises.namePlaceholder', 'Ex: Supino Reto')}
                />
                <TextField
                    label={t('exercises.descriptionLabel', 'Descrição')}
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                    placeholder={t('exercises.descriptionPlaceholder', 'Instruções da execução')}
                />
                <div className="form-row">
                    <div className="form-group">
                        <label>{t('exercises.categoryLabel', 'Categoria')}</label>
                        <select
                            className="gui-select-custom"
                            value={category}
                            onChange={(e) => setCategory(e.target.value)}
                        >
                            {categories.map((c) => (
                                <option key={c.value} value={c.value}>{c.label}</option>
                            ))}
                        </select>
                    </div>
                    <div className="form-group">
                        <label>{t('exercises.visibilityLabel', 'Visibilidade')}</label>
                        <VisibilityDropdown
                            value={visibility}
                            onChange={setVisibility}
                        />
                    </div>
                </div>
                <div className="form-row">
                    <TextField
                        type="number"
                        label={t('exercises.setsLabel', 'Séries')}
                        value={sets}
                        onChange={(e) => setSets(e.target.value)}
                        min={1}
                    />
                    <TextField
                        type="number"
                        label={t('exercises.repsLabel', 'Repetições / Duração (s)')}
                        value={repsOrDuration}
                        onChange={(e) => setRepsOrDuration(e.target.value)}
                        min={1}
                    />
                </div>
                <div className="modal-actions">
                    <Button type="button" variant="secondary" onClick={onClose}>
                        {t('application.button.cancel', 'Cancelar')}
                    </Button>
                    <Button type="submit" disabled={loading || !name.trim()}>
                        {loading ? t('application.loading', 'Salvando...') : t('application.button.confirm', 'Salvar')}
                    </Button>
                </div>
            </form>
        </Modal>
    );
};

export default ExerciseModal;
