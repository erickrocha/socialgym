# Transferências internacionais — gate de produção

Status: **preencher e obter assinatura antes do go-live quando qualquer destino for estrangeiro**.

## Inventário por ambiente

| Componente | Fornecedor | Região/país | Dados | Suboperadores | Salvaguarda/evidência |
|---|---|---|---|---|---|
| PostgreSQL | PENDENTE | PENDENTE | cadastro, consentimentos, treino | PENDENTE | PENDENTE |
| MongoDB | PENDENTE | PENDENTE | feed, evolução, denúncias | PENDENTE | PENDENTE |
| S3/CloudFront | PENDENTE | `WORKOUT_AWS_REGION` / `TIMELINE_AWS_REGION` | mídias e exportações | PENDENTE | PENDENTE |
| Logs/backups | PENDENTE | PENDENTE | metadados de acesso e cópias | PENDENTE | PENDENTE |

## Gate

Se algum país for diferente do Brasil, Jurídico/DPO deve registrar o mecanismo do art. 33 da LGPD aplicável, aprovar cláusulas-padrão contratuais e confirmar transparência na Política de Privacidade. Engenharia anexa contrato, lista de suboperadores, localização de backup/DR e avaliação de acesso governamental. Nenhum valor `PENDENTE` pode permanecer no release de produção.

Mudança de região, fornecedor ou suboperador exige nova revisão antes do deploy.
