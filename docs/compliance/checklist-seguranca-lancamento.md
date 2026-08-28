# Checklist de segurança — lançamento Brasil

Um item sem evidência é considerado não concluído.

- [ ] Termos, Privacidade, consentimento de saúde, DPO e versões aprovados pelo Jurídico.
- [ ] Testes provam cadastro atômico, bloqueio de menor de 18 anos e 403 sem consentimento atual.
- [ ] `AUTH_RULES_ENABLED=true`, bcrypt validado, lockout e revogação de token ativos.
- [ ] Segredos fora do repositório e rotacionados; `INTERNAL_SERVICE_SECRET` igual nos serviços.
- [ ] TLS público e gRPC validados; HSTS ativo.
- [ ] PostgreSQL, MongoDB, backups e S3 usam criptografia em repouso.
- [ ] Bucket bloqueia acesso público, usa política de menor privilégio e criptografia padrão; URLs de exportação expiram em 15 minutos e objetos em 7 dias.
- [ ] Regiões de todos os dados preenchidas e salvaguardas internacionais assinadas quando aplicável.
- [ ] Logs Nginx JSON chegam ao coletor sem corpo/query/token e têm retenção automática de **183 dias**.
- [ ] Alerta para falhas de exportação, purga, autenticação e acesso anormal ao S3 configurado.
- [ ] Restauração de backup testada e RPO/RTO aprovados.
- [ ] Runbook exercitado e contatos de plantão preenchidos.
- [ ] Conta moderadora provisionada pelo binário `grant_role`; acesso comum à fila retorna 403.
- [ ] Exportação contém Workout, Timeline e mídias; exclusão de teste não deixa dados consultáveis nos dois bancos/S3.
- [ ] Auditoria automatizada WCAG 2.1 A passa em cadastro, login e feed.

## Configuração do coletor de logs

O Nginx envia `compliance_json` para stdout. No CloudWatch Logs, configure o grupo de produção com retenção de 6 meses/183 dias (ou controle equivalente aprovado) e registre a captura da política como evidência do release. O `json-file` local do Docker é apenas buffer rotativo, não satisfaz a retenção legal.
