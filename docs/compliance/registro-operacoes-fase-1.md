# Registro de operações de tratamento — Fase 1 Brasil

Status: **minuta técnica; aprovação jurídica obrigatória antes do go-live**.

| Categoria | Titular | Finalidade | Hipótese/base a validar | Sistemas/operadores | Retenção e descarte |
|---|---|---|---|---|---|
| Cadastro, autenticação e idade | Pessoa usuária | Criar e proteger a conta; bloquear menores de 18 anos | Execução de contrato, legítimo interesse de segurança | Workout/PostgreSQL, provedor de infraestrutura | Vida da conta + prazo legal aprovado; expurgo pela exclusão da conta |
| Aceites legais | Pessoa usuária | Demonstrar versão, data e origem do aceite | Obrigação legal/regulatória e consentimento, conforme documento | Workout/PostgreSQL | Prazo probatório a definir pelo jurídico; revogações permanecem auditáveis até o expurgo aprovado |
| Medidas e evolução | Pessoa usuária | Registro opcional de bem-estar | Consentimento específico para dado sensível | Workout/PostgreSQL e Timeline/MongoDB | Enquanto houver consentimento/conta; bloqueio imediato de novas gravações após revogação |
| Perfil social e conteúdo | Pessoas usuárias e terceiros retratados | Feed, comentários, reações e mídia | Execução de contrato/consentimento, a validar | Timeline/MongoDB, S3/CloudFront | Vida da conta ou prazo de moderação; exclusão/anonimização conforme necessidade legal |
| Denúncias e decisões | Denunciante, autor e possível vítima | Segurança, defesa de direitos e moderação | Legítimo interesse, obrigação legal e exercício regular de direitos | Timeline/MongoDB | Prazo probatório a definir; histórico de decisão é imutável na aplicação |
| Treinos e relações profissionais | Pessoa usuária/profissional | Planejamento de treino e vínculos explicitamente aceitos | Execução de contrato; papéis controlador/operador a validar | Workout/PostgreSQL | Vida da conta e contrato; expurgo pela exclusão |
| Arquivos de exportação | Pessoa usuária | Atender portabilidade/acesso | Cumprimento da LGPD | S3 | 7 dias; worker remove objeto e marca a solicitação expirada |
| Logs de acesso | Visitante/pessoa usuária | Segurança e cumprimento do Marco Civil | Obrigação legal | Nginx stdout + coletor de logs | 183 dias; expurgo automático no provedor de logs |

## Fluxos internacionais

As regiões reais de PostgreSQL, MongoDB, S3, CloudFront, observabilidade e backup devem constar no inventário de produção. O deploy exige `WORKOUT_AWS_REGION` e `TIMELINE_AWS_REGION` explícitos. Qualquer região fora do Brasil só pode ser liberada após a salvaguarda prevista em `transferencias-internacionais.md`.

## Evidências técnicas

- Aceites: tabela `consent`, documentos versionados e endpoints de aceite/revogação.
- Direitos: fila `data_export`, ZIP temporário e purga coordenada entre PostgreSQL, MongoDB e S3.
- Logs: formato `compliance_json` no Nginx, sem query string.
- Conteúdo: fila `content_report`, decisão por papel `moderator` e histórico de auditoria.

Revisar este registro a cada mudança de finalidade, fornecedor, região, categoria de dado ou prazo de retenção.
