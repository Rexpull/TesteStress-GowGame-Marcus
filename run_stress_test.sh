#!/bin/bash

echo "🚀 Executando Teste de Stress do Desafio GOW"
echo "============================================="
echo

echo "📋 Verificando serviços..."
docker-compose ps

echo
echo "⚡ Executando teste de stress (50 usuários por 5 segundos)..."
docker run --rm -v $(pwd):/app --network host grafana/k6 run /app/tests/desafio_gow.js

echo
echo "📊 Verificando total de programadores inseridos..."
docker run --rm -v $(pwd):/app --network host grafana/k6 run --vus 1 --iterations 1 /app/tests/desafio_gow.js -e "http://localhost:9999/contagem-programadores" --quiet

echo
echo "✅ Teste concluído!" 