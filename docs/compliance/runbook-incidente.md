# Runbook de incidente de segurança e dados pessoais

Status: **controle operacional obrigatório para produção**.

## Papéis de plantão

Antes do lançamento, preencher no diretório corporativo: Incident Commander, Segurança, Engenharia, Encarregado/DPO, Jurídico, Comunicação e suplentes. A ausência de qualquer titular é bloqueador.

## Resposta

1. **Detectar e registrar (0–15 min):** abrir incidente, preservar horário, alertas, `requestId`, sistemas, região e primeira evidência. Não copiar dados pessoais para chats.
2. **Conter (até 60 min):** revogar credenciais/tokens afetados, isolar componente, bloquear chave ou rota e preservar evidência forense. Não destruir logs.
3. **Avaliar (início em até 4 h):** identificar categorias e volume de titulares, dados sensíveis, criptografia, possibilidade de dano, países, terceiros e período de exposição.
4. **Erradicar e recuperar:** corrigir a causa, rotacionar segredos, restaurar de fonte validada, testar e manter monitoramento reforçado.
5. **Decidir notificações:** DPO e Jurídico documentam a decisão e aplicam os prazos e o conteúdo vigentes da ANPD. Quando notificável, comunicar ANPD e titulares sem atraso indevido pelos canais aprovados.
6. **Encerrar:** relatório de causa raiz, linha do tempo, dados afetados, decisões, evidências, ações corretivas, responsáveis e datas. Revisão pós-incidente em até 5 dias úteis.

## Evidência mínima

- Não registrar token, senha, corpo de requisição, dado de saúde ou query string.
- Preservar logs de aplicação, acesso, banco, storage, IAM e deploy com cadeia de custódia.
- Toda consulta e exportação emergencial deve ter solicitante, finalidade, escopo e horário.

## Exercício

Executar tabletop antes do go-live e semestralmente. Cenário mínimo: URL de exportação exposta ou mídia íntima publicada; medir detecção, contenção, escalonamento ao DPO e comunicação.
