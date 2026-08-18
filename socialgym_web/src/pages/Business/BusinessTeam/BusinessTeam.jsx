import { useState, useEffect, useRef, useCallback } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { useSelector } from 'react-redux';
import { AppHeader, Button, Modal, TextField, Avatar, Spinner, Toast } from '../../../commons/gui';
import axios from '../../../axios.config';
import { activateBusinessProfileApi, deactivateBusinessProfileApi } from '../../../service/auth/auth.service';
import {
    getTeamMembersApi,
    sendTeamMemberRequestApi,
    cancelTeamMemberRequestApi,
} from '../../../service/team/team.service';
import { searchPersonsApi } from '../../../service/person/person.service';
import './BusinessTeam.scss';

const personName = (person) => [person.firstname, person.surname].filter(Boolean).join(' ') || person.uuid;

export const BusinessTeam = () => {
    const { id } = useParams();
    const { t } = useTranslation();
    const navigate = useNavigate();
    const person = useSelector((state) => state.person?.person);

    const [business, setBusiness] = useState(null);
    const [members, setMembers] = useState([]);
    const [sentRequests, setSentRequests] = useState([]);
    const [loading, setLoading] = useState(true);
    const [toastMessage, setToastMessage] = useState(null);
    const [notOwner, setNotOwner] = useState(false);
    const [busyPersonIds, setBusyPersonIds] = useState([]);

    const [isAddModalOpen, setIsAddModalOpen] = useState(false);
    const [searchQuery, setSearchQuery] = useState('');
    const [searchResults, setSearchResults] = useState([]);
    const [searching, setSearching] = useState(false);

    const activatedRef = useRef(false);

    const setPersonBusy = (personId, busy) => {
        setBusyPersonIds((prev) => (busy ? [...prev, personId] : prev.filter((pid) => pid !== personId)));
    };

    const loadTeam = useCallback(async () => {
        const page = await getTeamMembersApi();
        setMembers(page.members || []);
        setSentRequests(page.sentRequests || []);
    }, []);

    useEffect(() => {
        let cancelled = false;

        const init = async () => {
            setLoading(true);
            try {
                const { data: ownedProfiles } = await axios.get('/workout/api/business-profiles');
                const target = ownedProfiles.find((p) => String(p.id) === String(id));

                if (!target) {
                    if (!cancelled) setNotOwner(true);
                    return;
                }

                await activateBusinessProfileApi(target.uuid);
                activatedRef.current = true;

                const [{ data: activeProfile }] = await Promise.all([
                    axios.get('/workout/api/business-profiles/active'),
                    loadTeam(),
                ]);

                if (!cancelled) setBusiness(activeProfile);
            } catch (err) {
                console.error(err);
                if (!cancelled) {
                    setToastMessage({ type: 'error', message: t('team.errorLoading', 'Não foi possível carregar a equipe') });
                }
            } finally {
                if (!cancelled) setLoading(false);
            }
        };

        init();

        return () => {
            cancelled = true;
            if (activatedRef.current) {
                deactivateBusinessProfileApi().catch((err) => console.error(err));
                activatedRef.current = false;
            }
        };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [id]);

    useEffect(() => {
        if (!isAddModalOpen) {
            setSearchQuery('');
            setSearchResults([]);
            return;
        }
        if (searchQuery.trim().length < 2) {
            setSearchResults([]);
            return;
        }
        setSearching(true);
        const handle = setTimeout(async () => {
            try {
                const results = await searchPersonsApi(searchQuery.trim());
                setSearchResults(results);
            } catch (err) {
                console.error(err);
            } finally {
                setSearching(false);
            }
        }, 300);
        return () => clearTimeout(handle);
    }, [searchQuery, isAddModalOpen]);

    const handleInvite = async (candidate) => {
        setPersonBusy(candidate.id, true);
        try {
            await sendTeamMemberRequestApi(candidate.id);
            setToastMessage({ type: 'success', message: t('team.inviteSent', 'Convite enviado') });
            setSearchResults((prev) => prev.filter((p) => p.id !== candidate.id));
            await loadTeam();
        } catch (err) {
            console.error(err);
            setToastMessage({ type: 'error', message: t('team.errorInviting', 'Não foi possível enviar o convite') });
        } finally {
            setPersonBusy(candidate.id, false);
        }
    };

    const handleCancelRequest = async (candidate) => {
        setPersonBusy(candidate.id, true);
        try {
            await cancelTeamMemberRequestApi(candidate.id);
            setToastMessage({ type: 'info', message: t('team.inviteCancelled', 'Convite cancelado') });
            await loadTeam();
        } catch (err) {
            console.error(err);
            setToastMessage({ type: 'error', message: t('team.errorCancelling', 'Não foi possível cancelar o convite') });
        } finally {
            setPersonBusy(candidate.id, false);
        }
    };

    if (notOwner) {
        return (
            <div className="business-team-page">
                <AppHeader person={person} />
                <main className="team-container">
                    <p>{t('team.notOwner', 'Você não tem permissão para gerenciar a equipe deste perfil empresarial.')}</p>
                    <Button type="button" variant="secondary" onClick={() => navigate(`/business/${id}`)}>
                        ← {t('business.backToProfile', 'Voltar ao Perfil Empresarial')}
                    </Button>
                </main>
            </div>
        );
    }

    return (
        <div className="business-team-page">
            <AppHeader person={person} />
            {toastMessage && (
                <Toast
                    type={toastMessage.type}
                    message={toastMessage.message}
                    onClose={() => setToastMessage(null)}
                />
            )}
            <main className="team-container">
                <div className="team-header">
                    <div>
                        <Button type="button" variant="secondary" onClick={() => navigate(`/business/${id}`)}>
                            ← {t('business.backToProfile', 'Voltar ao Perfil Empresarial')}
                        </Button>
                        <h1>{business ? business.businessName : 'SocialGym Business'} - {t('team.pageTitle', 'Equipe e Colaboradores')}</h1>
                        <p>{t('team.pageSubtitle', 'Gerencie instrutores, treinadores e administradores da sua academia/estúdio')}</p>
                    </div>
                    <Button type="button" onClick={() => setIsAddModalOpen(true)} disabled={loading}>
                        + {t('team.addMemberBtn', 'Adicionar Membro')}
                    </Button>
                </div>

                {loading ? (
                    <Spinner />
                ) : (
                    <>
                        <section className="team-section">
                            <h2>{t('team.membersTitle', 'Membros')}</h2>
                            {members.length === 0 ? (
                                <p className="team-empty">{t('team.noMembers', 'Nenhum membro na equipe ainda.')}</p>
                            ) : (
                                <div className="team-grid">
                                    {members.map((member) => (
                                        <div key={member.id} className="team-card">
                                            <Avatar image={member.avatar} gender={member.gender} size="md" />
                                            <div className="member-info">
                                                <h3>{personName(member)}</h3>
                                            </div>
                                        </div>
                                    ))}
                                </div>
                            )}
                        </section>

                        <section className="team-section">
                            <h2>{t('team.pendingTitle', 'Convites pendentes')}</h2>
                            {sentRequests.length === 0 ? (
                                <p className="team-empty">{t('team.noPendingRequests', 'Nenhum convite pendente.')}</p>
                            ) : (
                                <div className="team-grid">
                                    {sentRequests.map((candidate) => (
                                        <div key={candidate.id} className="team-card">
                                            <Avatar image={candidate.avatar} gender={candidate.gender} size="md" />
                                            <div className="member-info">
                                                <h3>{personName(candidate)}</h3>
                                                <span className="role-badge">{t('team.pending', 'Pendente')}</span>
                                            </div>
                                            <Button
                                                type="button"
                                                variant="secondary"
                                                disabled={busyPersonIds.includes(candidate.id)}
                                                onClick={() => handleCancelRequest(candidate)}
                                            >
                                                {t('team.cancelInvite', 'Cancelar')}
                                            </Button>
                                        </div>
                                    ))}
                                </div>
                            )}
                        </section>
                    </>
                )}
            </main>

            {isAddModalOpen && (
                <Modal
                    isOpen={isAddModalOpen}
                    onClose={() => setIsAddModalOpen(false)}
                    title={t('team.addModalTitle', 'Convidar Novo Colaborador')}
                    isForm
                >
                    <div className="add-member-form">
                        <TextField
                            label={t('team.nameOrEmail', 'Nome ou E-mail')}
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            placeholder={t('team.searchPlaceholder', 'Buscar por nome ou e-mail')}
                        />

                        {searching && <Spinner />}

                        <div className="team-search-results">
                            {!searching && searchQuery.trim().length >= 2 && searchResults.length === 0 && (
                                <p className="team-empty">{t('team.noSearchResults', 'Nenhuma pessoa encontrada.')}</p>
                            )}
                            {searchResults.map((candidate) => (
                                <div key={candidate.id} className="team-search-result">
                                    <Avatar image={candidate.avatar} gender={candidate.gender} size="sm" />
                                    <span className="team-search-result__name">{personName(candidate)}</span>
                                    <Button
                                        type="button"
                                        variant="primary"
                                        disabled={busyPersonIds.includes(candidate.id)}
                                        onClick={() => handleInvite(candidate)}
                                    >
                                        {t('team.sendInvite', 'Enviar Convite')}
                                    </Button>
                                </div>
                            ))}
                        </div>

                        <div className="modal-actions">
                            <Button type="button" variant="secondary" onClick={() => setIsAddModalOpen(false)}>
                                {t('application.button.cancel', 'Cancelar')}
                            </Button>
                        </div>
                    </div>
                </Modal>
            )}
        </div>
    );
};

export default BusinessTeam;
