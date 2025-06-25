# 🧪 Guia de Teste Manual - API GOW

Este documento contém comandos para testar manualmente os endpoints da API do Desafio GOW.

## 🚀 Iniciando os Serviços

```bash
# Iniciar todos os serviços
docker-compose up --build -d

# Verificar se estão rodando
docker-compose ps

# Ver logs em tempo real
docker-compose logs -f
```

## 📝 Testando POST /programadores

### Teste 1: Criação válida
```bash
curl -X POST http://localhost:8082/programadores \
  -H "Content-Type: application/json" \
  -d '{
    "apelido": "joao123",
    "nome": "João Silva",
    "nascimento": "1990-05-15",
    "stack": ["Rust", "PostgreSQL", "Docker"]
  }'
```

**Resultado esperado**: Status 201 com header Location

### Teste 2: Criação sem stack (válida)
```bash
curl -X POST http://localhost:8082/programadores \
  -H "Content-Type: application/json" \
  -d '{
    "apelido": "maria456",
    "nome": "Maria Santos",
    "nascimento": "1985-12-20",
    "stack": null
  }'
```

**Resultado esperado**: Status 201

### Teste 3: Apelido duplicado (deve falhar)
```bash
curl -X POST http://localhost:8082/programadores \
  -H "Content-Type: application/json" \
  -d '{
    "apelido": "joao123",
    "nome": "Outro João",
    "nascimento": "1992-03-10",
    "stack": ["Java"]
  }'
```

**Resultado esperado**: Status 422 (Unprocessable Entity)

### Teste 4: Dados inválidos
```bash
# Apelido muito longo (deve falhar)
curl -X POST http://localhost:8082/programadores \
  -H "Content-Type: application/json" \
  -d '{
    "apelido": "este_apelido_tem_mais_de_32_caracteres_e_deve_falhar",
    "nome": "Teste",
    "nascimento": "1990-01-01",
    "stack": []
  }'
```

**Resultado esperado**: Status 400 (Bad Request)

### Teste 5: Campo obrigatório ausente
```bash
curl -X POST http://localhost:8082/programadores \
  -H "Content-Type: application/json" \
  -d '{
    "nome": "Sem Apelido",
    "nascimento": "1990-01-01",
    "stack": ["Rust"]
  }'
```

**Resultado esperado**: Status 400

### Teste 6: Data inválida
```bash
curl -X POST http://localhost:8082/programadores \
  -H "Content-Type: application/json" \
  -d '{
    "apelido": "teste_data",
    "nome": "Teste Data",
    "nascimento": "30-12-1990",
    "stack": ["Python"]
  }'
```

**Resultado esperado**: Status 400

## 📊 Testando GET /contagem-programadores

```bash
# Consultar contagem atual
curl http://localhost:8082/contagem-programadores
```

**Resultado esperado**: Número em texto plano (ex: "2")

## 🔥 Teste de Stress com k6

### Executar o teste oficial do desafio
```bash
# Dentro do container k6
docker-compose run --rm k6 k6 run /tests/desafio_interno_k6.js

# Ou com npm script
npm run test
```

### Executar teste local (se k6 estiver instalado)
```bash
k6 run tests/desafio_gow.js
```

## 📈 Monitoramento durante o teste

### Terminal 1: Logs da aplicação
```bash
docker-compose logs -f api1 api2
```

### Terminal 2: Estatísticas de recursos
```bash
docker stats
```

### Terminal 3: Contagem em tempo real
```bash
# Executar várias vezes durante o teste
while true; do
  echo "Contagem: $(curl -s http://localhost:8082/contagem-programadores)"
  sleep 1
done
```

## 🎯 Validação de Performance

Após o teste de stress (50 VUs por 5 segundos), verifique:

1. **Taxa de sucesso**: > 99% das requisições devem retornar 201
2. **Tempo de resposta**: P95 < 100ms
3. **Consistência**: Contagem final deve corresponder aos CREATEs bem-sucedidos
4. **Recursos**: Uso de CPU e RAM dentro dos limites

## 🐛 Solução de Problemas

### API não responde
```bash
# Verificar logs
docker-compose logs api1 api2

# Verificar se containers estão rodando
docker-compose ps

# Reiniciar se necessário
docker-compose restart api1 api2
```

### Erro de conexão com banco
```bash
# Verificar logs do PostgreSQL
docker-compose logs db

# Verificar se banco está acessível
docker-compose exec db psql -U postgres -c "SELECT 1;"
```

### Nginx retorna 502
```bash
# Verificar logs do nginx
docker-compose logs nginx

# Verificar upstreams
docker-compose exec nginx cat /etc/nginx/nginx.conf | grep server
```

## 🏆 Métricas Esperadas

Com esta implementação otimizada, esperamos:

- **Throughput**: > 1000 req/s
- **Taxa de erro**: < 1%
- **Tempo médio**: < 50ms
- **P95**: < 100ms
- **Uso de CPU**: < 80% do limite
- **Uso de RAM**: < 90% do limite

---

**Desenvolvido para o Desafio GOW - Teste de Stress** 