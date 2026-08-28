# Lançamento no Brasil — Backlog de conformidade (Fase 2)

> Não é parecer jurídico. Continuação de `lancamento-brasil-fase-1.md`. Estes
> itens assumem que os P0/P1 da Fase 1 já estão em produção. Foco: amadurecer a
> governança de privacidade, suportar novos atores (profissionais/empresas com
> acesso delegado) e escalar moderação e segurança.

## Legenda

- **P2** — Fase 2, planejada para o trimestre seguinte ao go-live.
- **P3** — depende de decisão de produto ainda não tomada (marcado explicitamente).

Cada item: objetivo · base legal · onde mexer · critério de aceite.

---

## Governança de privacidade

### P2-1. Registro das operações de tratamento (ROPA)
- **Objetivo:** inventário formal e versionado de todo tratamento: categorias de dados, finalidades, bases legais, retenção, operadores, transferências.
- **Base legal:** LGPD Art. 37.
- **Onde mexer:** documento vivo em `docs/` (ou ferramenta de GRC); cobrir `workout` (PostgreSQL/PostGIS), `timeline` (MongoDB), S3, SQS, e-mail transacional.
- **Critério de aceite:** ROPA aprovado pelo Encarregado, com processo de atualização a cada mudança de schema ou de fornecedor.

### P2-2. RIPD / DPIA para dados sensíveis e geolocalização precisa
- **Objetivo:** relatório de impacto à proteção de dados para: dados de saúde (composição corporal, circunferências, check-ins) e localização precisa (`person_address.location` PostGIS).
- **Base legal:** LGPD Art. 5º XVII, Art. 38.
- **Onde mexer:** documento em `docs/`; envolver jurídico e segurança; registrar medidas mitigadoras (minimização, pseudonimização, controle de acesso).
- **Critério de aceite:** RIPD concluído, com plano de ação para riscos residuais e revisão anual agendada.

### P2-3. Lista pública de sub-operadores
- **Objetivo:** publicar quem processa dados em nome da SocialGym (AWS S3/SQS, provedor de e-mail, analytics, push) e a finalidade.
- **Base legal:** LGPD Art. 6º VI; transparência.
- **Onde mexer:** página estática linkada na Política de Privacidade; processo de aviso prévio antes de trocar/adicionar operador.
- **Critério de aceite:** lista publicada e mantida junto com o ROPA.

### P2-4. Transferência internacional — formalização
- **Objetivo:** adotar o mecanismo de transferência conforme a regulamentação da ANPD (cláusulas-padrão contratuais) para todo fluxo que saia do Brasil.
- **Base legal:** LGPD Art. 33; Resolução CD/ANPD nº 19/2024.
- **Onde mexer:** contratos com AWS e demais operadores fora do país; menção do mecanismo na Política de Privacidade.
- **Critério de aceite:** cláusulas-padrão assinadas/aderidas e referenciadas no ROPA.

### P2-5. Automação do atendimento a titulares
- **Objetivo:** portal/painel do titular com autoatendimento para acesso, correção, exportação, exclusão e revogação de consentimento, dentro do prazo legal.
- **Base legal:** LGPD Art. 18 e §§; prazo de resposta.
- **Onde mexer:** evoluir os endpoints da Fase 1 (P0-4) para um fluxo assíncrono com status; UI dedicada no web e mobile; métricas de SLA.
- **Critério de aceite:** requisição de titular rastreável do início ao fim, com prazo monitorado e relatório mensal ao Encarregado.

### P2-6. Portabilidade em formato interoperável
- **Objetivo:** exportação da Fase 1 evolui para formato estruturado e documentado (esquema publicado), permitindo migração para outro serviço.
- **Base legal:** LGPD Art. 18 V.
- **Onde mexer:** definir esquema JSON estável de exportação agregando `workout` + `timeline`; versionar; documentar em `docs/`.
- **Critério de aceite:** arquivo de exportação valida contra esquema publicado; changelog de versões.

---

## Consentimento e granularidade

### P2-7. Preferências de privacidade granulares
- **Objetivo:** o titular controla a visibilidade de cada categoria (perfil, medidas, treinos, check-ins) — inclusive resolver o `Visibility::Professional` hoje sem público resolvível (ver `CLAUDE.md`).
- **Base legal:** LGPD Art. 6º (finalidade, adequação); autodeterminação informativa.
- **Onde mexer:** `settings` (`workout/entity/src/settings_entity.rs`) ou nova tabela de preferências; enforcement no `timeline` (feed, check-ins) e `workout` (perfil).
- **Critério de aceite:** cada categoria tem escolha de audiência aplicada nas APIs; padrão mais restritivo.

### P2-8. Campo "gênero customizado" como dado sensível
- **Objetivo:** tratar o texto livre de gênero (`person.gender` quando "custom") com a proteção de dado sensível (pode revelar identidade de gênero / vida sexual).
- **Base legal:** LGPD Art. 5º II, Art. 11.
- **Onde mexer:** marcar a coluna como sensível no ROPA; controle de acesso e minimização; considerar não expor em respostas públicas por padrão.
- **Critério de aceite:** campo não aparece em perfis públicos sem opt-in; acesso logado.

### P2-9. Comunicações de marketing e push — opt-in
- **Objetivo:** consentimento separado para e-mail/push promocional, com descadastro fácil; transacional continua sem opt-in.
- **Base legal:** LGPD Art. 7º/8º; CDC; boas práticas antisspam.
- **Onde mexer:** `settings.notifications_enabled` é genérico demais — separar canais e finalidades; link de descadastro em todo e-mail promocional.
- **Critério de aceite:** nenhum envio promocional sem opt-in explícito; descadastro em 1 clique honrado.

---

## Acesso delegado (profissionais e empresas)

### P2-10. Modelo de consentimento para "agir em nome de"
- **Objetivo:** quando o recurso for implementado, todo acesso de profissional/empresa a dados de uma pessoa exige grant explícito do titular, seguindo o padrão `friends.status` (request → accept) indicado no `CLAUDE.md`.
- **Base legal:** regra de domínio "autoridade é delegada por consentimento"; LGPD Art. 7º/11.
- **Onde mexer:** nova tabela de grants em `workout/migration`; use cases de solicitação/aceite/revogação; enforcement em toda leitura cross-perfil no `workout` e `timeline`.
- **Critério de aceite:** sem grant ativo, profissional não lê nenhum dado da pessoa; revogação corta o acesso imediatamente; testes cobrindo os dois serviços.

### P2-11. Contrato de tratamento (DPA) controlador/operador
- **Objetivo:** definir e contratualizar os papéis entre SocialGym, empresa (academia) e profissional para os dados de saúde de terceiros.
- **Base legal:** LGPD Art. 39; responsabilidade solidária Art. 42.
- **Onde mexer:** anexo de tratamento de dados aceito no onboarding de perfil Profissional/Empresa; registro de aceite.
- **Critério de aceite:** nenhum perfil de negócio opera sobre dados de pessoas sem DPA aceito.

### P2-12. Log de auditoria de acesso a dados sensíveis
- **Objetivo:** registrar quem (profissional/empresa) acessou quais dados de saúde de qual titular e quando; disponível ao titular.
- **Base legal:** LGPD Art. 6º VI e X (transparência, responsabilização).
- **Onde mexer:** trilha de auditoria no `workout`/`timeline` para leituras cross-perfil; visão "quem acessou meus dados" no app.
- **Critério de aceite:** cada acesso delegado gera registro imutável consultável pelo titular.

### P2-13. Verificação de credencial profissional
- **Objetivo:** validar registro CREF/CFN (ou outro conselho) antes de habilitar perfil Profissional a atender pessoas.
- **Base legal:** CDC; regulação profissional; mitigação de responsabilidade da plataforma.
- **Onde mexer:** campo e fluxo de verificação no onboarding de `business_profile` tipo `Professional`; estado "verificado".
- **Critério de aceite:** perfil não verificado não recebe grants de pessoas; status visível.

---

## Moderação e conteúdo (escala)

### P2-14. Moderação proativa e enforcement de Termos
- **Objetivo:** filtros automáticos para conteúdo ilícito (nudez não consentida, discurso de ódio, conteúdo com menores), fila de revisão humana, sanções graduais.
- **Base legal:** Marco Civil Art. 19/21; ECA; Termos de Uso.
- **Onde mexer:** `timeline` — pipeline de moderação em posts/comentários/mídia; painel interno; histórico de sanções por usuário.
- **Critério de aceite:** SLA por categoria; relatório de transparência interno mensal.

### P2-15. Retenção granular e expurgo automatizado
- **Objetivo:** prazo de guarda por tabela/coleção com job de expurgo/anonimização automático (evolui o P0-6 de logs para todo o dado).
- **Base legal:** LGPD Art. 15, 16.
- **Onde mexer:** política por entidade (`workout` e `timeline`); jobs agendados; registro de execução.
- **Critério de aceite:** cada categoria tem prazo definido e expurgo comprovável; conta inativa tratada conforme política.

---

## Segurança (amadurecimento)

### P2-16. MFA e criptografia de campo para dados sensíveis
- **Objetivo:** segundo fator opcional (recomendado para perfis de negócio); criptografia em nível de aplicação para os campos de saúde mais sensíveis.
- **Base legal:** LGPD Art. 46 (medidas técnicas adequadas ao risco).
- **Onde mexer:** fluxo de auth no `workout`; camada de cripto para `body_composition` / `circumferences` no `timeline` e `person_info.weight/height` no `workout`.
- **Critério de aceite:** MFA disponível e obrigatório para perfis de negócio; dados sensíveis não legíveis em dump de banco sem a chave.

### P2-17. Simulação de incidente (tabletop)
- **Objetivo:** exercitar o runbook de incidente da Fase 1 com cenário real de vazamento de dado de saúde.
- **Base legal:** LGPD Art. 48; princípio da responsabilização.
- **Onde mexer:** exercício conduzido por segurança + jurídico; ajustar runbook e prazos de notificação à ANPD.
- **Critério de aceite:** relatório do exercício com lições e correções aplicadas.

---

## Acessibilidade e marcas

### P2-18. Acessibilidade WCAG 2.1 AA (web e mobile)
- **Objetivo:** elevar da conformidade nível A (Fase 1) para AA nas jornadas principais; incluir apps Flutter.
- **Base legal:** Lei 13.146/2015; eMAG.
- **Onde mexer:** `socialgym_web`, `socialgym_mobile`, `lapidation_mobile`.
- **Critério de aceite:** auditoria AA sem erros nas jornadas de cadastro, login, feed, criação de post e check-in.

### P2-19. Revisão regulatória específica da marca "Lapidation Clinic"
- **Objetivo:** o nome "Clinic" sugere contexto clínico; revisar posicionamento, textos e funcionalidades do `lapidation_mobile` quanto a enquadramento como software em saúde.
- **Base legal:** ANVISA RDC 657/2022; publicidade de serviços de saúde (CFM/conselhos).
- **Onde mexer:** conteúdo e features do `lapidation_mobile`; Termos e Política próprios da marca.
- **Critério de aceite:** parecer jurídico específico para a marca antes de divulgá-la no Brasil.

---

## P3 — Depende de decisão de produto

### P3-1. Onboarding de adolescentes (13–17) com consentimento parental
- **Objetivo:** se produto decidir aceitar menores, implementar verificação de idade robusta e consentimento específico e destacado de ao menos um dos pais/responsável, com melhor interesse da criança.
- **Base legal:** LGPD Art. 14; ECA.
- **Onde mexer:** fluxo de cadastro condicional; verificação de responsável; limitação de recursos (ex.: sem localização precisa, sem descoberta pública).
- **Critério de aceite:** menor não conclui cadastro sem consentimento verificável do responsável; conjunto reduzido de funcionalidades.

### P3-2. Marketplace de profissionais e empresas
- **Objetivo:** ao construir "encontrar profissionais/empresas", revisar responsabilidade da plataforma como intermediária, publicidade, avaliações/reviews e dados expostos.
- **Base legal:** CDC (intermediação, propaganda); LGPD (dados de contato).
- **Onde mexer:** especificação do Marketplace (hoje só existe `header.marketplaceTitle`); Termos com regras de listagem e reviews.
- **Critério de aceite:** revisão jurídica do modelo de intermediação concluída antes do lançamento do recurso.
