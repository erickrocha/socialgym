import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Modal, TextField, Button, Uploader } from '../../gui';
import './EvolutionModal.scss';

export const EvolutionModal = ({ isOpen, onClose, onSubmit, loading = false }) => {
    const { t } = useTranslation();
    const [weight, setWeight] = useState('');
    const [bodyFat, setBodyFat] = useState('');
    const [notes, setNotes] = useState('');
    const [photoUrl, setPhotoUrl] = useState('');

    const handleSubmit = (e) => {
        e.preventDefault();
        if (!weight) return;
        onSubmit({
            weight: Number(weight),
            body_fat: bodyFat ? Number(bodyFat) : null,
            notes,
            photo_url: photoUrl,
            date: new Date().toISOString()
        });
        setWeight('');
        setBodyFat('');
        setNotes('');
        setPhotoUrl('');
    };

    if (!isOpen) return null;

    return (
        <Modal isOpen={isOpen} onClose={onClose} title={t('evolution.modalTitle', 'Novo Check-in de Evolução')}>
            <form onSubmit={handleSubmit} className="evolution-modal-form">
                <div className="form-row">
                    <TextField
                        type="number"
                        step="0.1"
                        label={t('evolution.weightLabel', 'Peso Atual (kg)')}
                        value={weight}
                        onChange={(e) => setWeight(e.target.value)}
                        required
                        placeholder="Ex: 75.5"
                    />
                    <TextField
                        type="number"
                        step="0.1"
                        label={t('evolution.bodyFatLabel', 'Gordura Corporal (%)')}
                        value={bodyFat}
                        onChange={(e) => setBodyFat(e.target.value)}
                        placeholder="Ex: 15.2"
                    />
                </div>
                <TextField
                    label={t('evolution.notesLabel', 'Notas e Observações')}
                    value={notes}
                    onChange={(e) => setNotes(e.target.value)}
                    placeholder={t('evolution.notesPlaceholder', 'Como você se sentiu hoje? Evolução de cargas ou dieta...')}
                />
                
                <div className="form-group">
                    <label>{t('evolution.photoLabel', 'Foto da Evolução (Opcional)')}</label>
                    <Uploader
                        onUploadSuccess={(url) => setPhotoUrl(url)}
                        placeholder={t('evolution.uploadPhoto', 'Enviar Foto de Progresso')}
                    />
                </div>

                <div className="modal-actions">
                    <Button type="button" variant="secondary" onClick={onClose}>
                        {t('application.button.cancel', 'Cancelar')}
                    </Button>
                    <Button type="submit" disabled={loading || !weight}>
                        {loading ? t('application.loading', 'Registrando...') : t('evolution.saveCheckin', 'Registrar Check-in')}
                    </Button>
                </div>
            </form>
        </Modal>
    );
};

export default EvolutionModal;
