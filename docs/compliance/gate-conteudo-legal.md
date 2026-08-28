# Gate de conteúdo legal e atendimento ao titular

Os arquivos em `workout/application/resources/legal/pt-BR/` são minutas técnicas. O pipeline de release deve falhar enquanto qualquer documento contiver `minuta`, `sujeita à aprovação`, `devem ser inseridos` ou `PENDENTE`.

Antes do go-live, Jurídico/DPO deve:

- aprovar Termos, Política de Privacidade e consentimento destacado de saúde;
- preencher nome e contato público do Encarregado e o canal alternativo;
- definir SLA operacional de requisições e denúncia de imagem íntima;
- aprovar base legal, finalidade e retenção do registro de operações;
- incluir regiões, transferências internacionais e salvaguardas;
- validar cláusulas de profissionais/empresas, suspensão, foro, responsabilidade e CDC;
- incrementar a variável de versão correspondente sempre que o conteúdo exigir novo aceite.

Atendimento registra protocolo, identidade verificada, escopo, decisões, resposta e data. Solicitações de acesso usam a exportação assíncrona; exclusão usa o fluxo de conta e registra exceções legais documentadas.
