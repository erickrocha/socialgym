# Lançamento no Brasil — Backlog de conformidade (Fase 1)

> Não é parecer jurídico. Lista técnica derivada do código atual, para validação
> com assessoria jurídica antes do go-live. Base legal citada: LGPD (Lei
> 13.709/2018), Marco Civil da Internet (Lei 12.965/2014), ECA, CDC, Lei
> Brasileira de Inclusão (Lei 13.146/2015).

## Legenda

- **P0** — bloqueador. Não lançar sem isso.
- **P1** — Fase 1, entra junto ou logo após o go-live com prazo curto acordado com o jurídico.
- **P2** — pós-lançamento (fora deste documento; ver análise longa).

Cada item: objetivo · base legal · onde mexer · critério de aceite.

---

## P0 — Bloqueadores

### P0-1. Aceite de Termos de Uso e Política de Privacidade no cadastro
- **Objetivo:** registrar aceite versionado (titular, versão do documento, timestamp, IP) e permitir revogação.
- **Base legal:** LGPD Art. 7º, 8º, 9º; transparência Art. 6º VI.
- **Onde mexer:**
  - Conteúdo: páginas estáticas PT-BR no `socialgym_web` e telas equivalentes em `socialgym_mobile` / `lapidation_mobile`.
  - Backend: nova migration em `workout/migration/src/` para tabela `consent` (`person_id`, `document`, `version`, `accepted_at`, `ip`, `revoked_at`); entity + gateway + use case.
  - `workout/application/src/http/auth_controller.rs::sign_up` e `SignUpJson`: exigir `terms_version` / `privacy_version` aceitos; recusar cadastro sem eles.
  - `socialgym_web/src/pages/security/SignUp/SignUp.jsx`: checkbox obrigatório com links.
- **Critério de aceite:** cadastro sem aceite retorna 400; registro de consentimento gravado e consultável; endpoint para revogar.

### P0-2. Consentimento destacado para dados de saúde
- **Objetivo:** consentimento específico e separado do aceite geral para dados sensíveis (composição corporal, medidas, check-ins de evolução, peso/altura). App utilizável sem fornecê-los.
- **Base legal:** LGPD Art. 11, I (consentimento destacado para dado sensível).
- **Onde mexer:**
  - Reutilizar a tabela `consent` do P0-1 com `document = "health_data"`.
  - `timeline`: bloquear criação/edição de `evolution_check_in`, `body_composition`, `circumferences` (ver `timeline/domain/src/`) sem consentimento ativo — checar via gRPC no `workout` ou replicar flag.
  - `workout`: `person_info.weight` / `person_info.height` idem no use case que os grava.
  - Front/mobile: fluxo de opt-in próprio antes da primeira tela de medidas.
- **Critério de aceite:** sem consentimento de saúde, as telas de medição ficam indisponíveis e as APIs retornam 403; demais funções sociais funcionam normalmente.

### P0-3. Verificação de idade / menores
- **Objetivo:** impedir cadastro de menor de 18 anos (decisão Fase 1), com validação no backend.
- **Base legal:** LGPD Art. 14; ECA; capacidade civil (CC Art. 3º/4º).
- **Onde mexer:**
  - `workout/application/src/http/auth_controller.rs::sign_up` (ou `PersonUseCase::add`): calcular idade a partir de `date_of_birth`; recusar < 18.
  - `socialgym_web/src/pages/security/SignUp/SignUp.jsx`: validação client-side + mensagem.
  - Termos: cláusula de que o serviço é para maiores de 18.
- **Critério de aceite:** data de nascimento que resulte em < 18 anos retorna 400 com chave de erro específica; teste em `workout/business/tests/mock.rs`.

### P0-4. Direitos do titular — acesso e exclusão efetivos
- **Objetivo:** (a) exportar todos os dados do titular em formato legível; (b) garantir que a exclusão apaga/anonimiza dados nos dois serviços e no S3.
- **Base legal:** LGPD Art. 18, II, V e VI.
- **Onde mexer:**
  - Exclusão: `workout/business/src/use_cases/account_deletion_use_case.rs` já existe (colunas `deletion_requested_at` / `deletion_scheduled_at`). Estender para: remover `person_media` do bucket S3, acionar limpeza no `timeline` (posts, comentários, reações, check-ins, notificações no MongoDB), anonimizar autoria de conteúdo que deva permanecer.
  - Exportação: novo use case + endpoint em `workout` agregando dados locais; `timeline` expõe export próprio (REST) ou via gRPC; cliente/back consolida em um arquivo (JSON/ZIP).
  - Front/mobile: telas "Baixar meus dados" e "Excluir conta" com prazo informado.
- **Critério de aceite:** solicitação de exportação gera arquivo com dados de `workout` + `timeline`; após a exclusão agendada, nenhum registro pessoal ou mídia permanece consultável; teste cobrindo a limpeza de S3 e timeline.

### P0-5. Encarregado (DPO) e canal do titular
- **Objetivo:** designar Encarregado e publicar contato; canal para requisições de titular.
- **Base legal:** LGPD Art. 41 e 18 §2º.
- **Onde mexer:** dado na Política de Privacidade e numa página "Privacidade / Fale com o Encarregado" no web e mobile; e-mail dedicado com fluxo interno de atendimento e SLA.
- **Critério de aceite:** contato do Encarregado visível sem login; processo interno documentado com prazo de resposta.

### P0-6. Base legal, finalidade e retenção documentadas + expurgo de logs
- **Objetivo:** definir por categoria de dado a base legal, a finalidade e o prazo de guarda; implementar expurgo.
- **Base legal:** LGPD Art. 6º, 15, 16, 37; Marco Civil Art. 15 (logs de acesso a aplicação = 6 meses).
- **Onde mexer:**
  - Documento interno (registro das operações de tratamento) — pode viver em `docs/`.
  - Retenção de logs de acesso: rotina de expurgo na infra/serviço de log; não guardar além do necessário nem menos que 6 meses.
- **Critério de aceite:** tabela de retenção aprovada pelo jurídico; job de expurgo rodando; registro das operações versionado.

### P0-7. Segurança e resposta a incidente
- **Objetivo:** confirmar controles mínimos e ter plano de notificação de incidente.
- **Base legal:** LGPD Art. 46, 48 e 49.
- **Onde mexer:**
  - Confirmar hash de senha (bcrypt/argon2) no fluxo de `user` — já há validação de senha fraca, lockout (`m20260813_000001`) e revogação de token (`revoked_token`), verificar o algoritmo.
  - Criptografia em repouso do bucket S3; TLS já usado nos gateways (`infra/certs`).
  - Runbook de incidente: detecção, contenção, avaliação de risco, notificação à ANPD e aos titulares, prazos.
- **Critério de aceite:** checklist de segurança revisado; runbook de incidente em `docs/` com responsáveis e prazos.

### P0-8. Transferência internacional de dados
- **Objetivo:** fixar/localizar onde os dados são processados e armazenados; se fora do Brasil, adotar salvaguarda e informar.
- **Base legal:** LGPD Art. 33.
- **Onde mexer:**
  - Definir região do bucket (`AWS_WORKOUT_BUCKET` em `workout/.env`; uso em `sqs_consumer_use_case.rs` / `s3_gateway.rs`) e do banco/infra.
  - Se houver transferência internacional: cláusulas-padrão contratuais e menção expressa na Política de Privacidade.
- **Critério de aceite:** região documentada; se aplicável, salvaguarda assinada e descrita na política.

---

## P1 — Fase 1 (curto prazo)

### P1-1. Denúncia e remoção de conteúdo (UGC)
- **Objetivo:** botão de denúncia em post/comentário/mídia e fluxo de análise/remoção; remoção prioritária de imagem íntima sob notificação.
- **Base legal:** Marco Civil Art. 19 e 21; direito de imagem (CF Art. 5º X, CC Art. 20).
- **Onde mexer:** `timeline` (posts, comentários, mídia) — endpoint de denúncia, estado de moderação, remoção; tela no web/mobile; caixa de entrada de moderação interna.
- **Critério de aceite:** conteúdo denunciado entra em fila; remoção registra motivo e autor; SLA definido para casos de imagem íntima.

### P1-2. Consentimento de imagem de terceiros
- **Objetivo:** ao publicar foto/vídeo, exigir declaração de que há consentimento das pessoas retratadas.
- **Base legal:** CC Art. 20; LGPD (terceiro como titular).
- **Onde mexer:** cláusula nos Termos + checkbox/aviso no upload (`person_media` / fluxo de post no `timeline`).
- **Critério de aceite:** upload sem a confirmação é bloqueado no cliente; cláusula publicada.

### P1-3. Banner de cookies / rastreadores (web)
- **Objetivo:** inventariar rastreadores do `socialgym_web` e obter consentimento antes de disparar analytics não essencial.
- **Base legal:** LGPD Art. 7º/8º; Guia de Cookies da ANPD.
- **Onde mexer:** `socialgym_web` — banner com opção de recusar não essenciais; carregar analytics só após opt-in.
- **Critério de aceite:** sem interação, nenhum cookie/rastreador não essencial é setado; escolha persistida.

### P1-4. Posicionamento "bem-estar", não diagnóstico
- **Objetivo:** revisar textos de UI e marketing para não sugerir diagnóstico/tratamento; apresentar métricas (ex.: gordura visceral) como registro informado pelo usuário.
- **Base legal:** ANVISA RDC 657/2022 (software como dispositivo médico) — evitar enquadramento.
- **Onde mexer:** locales (`socialgym_web/src/locales/pt-BR`, mobile), telas de composição corporal e check-in, materiais de marketing.
- **Critério de aceite:** revisão feita e aprovada pelo jurídico; nenhuma tela faz interpretação clínica automática.

### P1-5. Termos para atuação de profissionais e empresas
- **Objetivo:** ToS específico exigindo registro profissional válido (CREF/CFN) e disclaimer de responsabilidade técnica do profissional; contrato/DPA para empresas (academias).
- **Base legal:** CDC; LGPD (controlador/operador Art. 5º VI/VII e Art. 39); regulação profissional.
- **Onde mexer:** conteúdo de Termos; quando o "agir em nome de" for implementado, seguir o padrão de consentimento `friends.status` (request → accept) já indicado no `CLAUDE.md`.
- **Critério de aceite:** Termos publicados; nenhum acesso de profissional a dados de pessoa sem grant explícito (a validar na implementação do recurso).

### P1-6. Acessibilidade mínima (web)
- **Objetivo:** conformidade básica WCAG 2.1 nível A nas telas públicas e principais fluxos.
- **Base legal:** Lei 13.146/2015 (Art. 63); Decreto 5.296/2004.
- **Onde mexer:** `socialgym_web` — labels em formulários (ex.: `SignUp.jsx`), contraste, navegação por teclado, textos alternativos em imagens.
- **Critério de aceite:** auditoria automatizada sem erros de nível A nos fluxos de cadastro, login e feed.

### P1-7. Cláusulas contratuais essenciais nos Termos
- **Objetivo:** lei aplicável (brasileira) e foro, limitação de responsabilidade, regras de suspensão/encerramento de conta, e — se houver recurso pago — preço claro, direito de arrependimento de 7 dias, cancelamento e reembolso.
- **Base legal:** CDC Art. 49; CC.
- **Onde mexer:** documento de Termos de Uso.
- **Critério de aceite:** Termos revisados pelo jurídico e publicados antes do go-live.

---

## Itens deixados para P2 (não nesta fase)

RIPD/DPIA formal para dados de saúde e geolocalização precisa; registro completo das operações de tratamento junto à ANPD; consentimento parental do Art. 14 caso decida-se aceitar 13–17 anos; tratamento específico do campo livre "gênero customizado" como dado sensível; políticas de retenção granulares por tabela.
