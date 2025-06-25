#!/bin/bash

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Desafio GOW - Teste de Stress${NC}"
echo -e "${BLUE}Implementação em Rust + Actix-Web + PostgreSQL${NC}"
echo ""

case $1 in
  "build")
    echo -e "${YELLOW}🔨 Construindo containers...${NC}"
    docker-compose build --no-cache
    ;;
  "up"|"start")
    echo -e "${GREEN}🌟 Iniciando serviços...${NC}"
    docker-compose up --build -d
    echo -e "${GREEN}✅ Serviços iniciados!${NC}"
    echo -e "${BLUE}🌐 API disponível em: http://localhost:8082${NC}"
    echo -e "${BLUE}📊 Para executar o teste: ./run.sh test${NC}"
    ;;
  "down"|"stop")
    echo -e "${YELLOW}⏹️  Parando serviços...${NC}"
    docker-compose down
    ;;
  "test")
    echo -e "${YELLOW}🧪 Executando teste de stress...${NC}"
    docker-compose run --rm k6 k6 run /tests/desafio_interno_k6.js
    ;;
  "test-local")
    echo -e "${YELLOW}🧪 Executando teste local...${NC}"
    k6 run tests/desafio_gow.js
    ;;
  "count")
    echo -e "${BLUE}📊 Consultando contagem de programadores...${NC}"
    curl -s http://localhost:8082/contagem-programadores
    echo ""
    ;;
  "logs")
    docker-compose logs -f
    ;;
  "status")
    echo -e "${BLUE}📊 Status dos serviços:${NC}"
    docker-compose ps
    ;;
  "stats")
    echo -e "${BLUE}📊 Estatísticas de recursos:${NC}"
    docker stats --format 'table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}\t{{.BlockIO}}'
    ;;
  "clean")
    echo -e "${RED}🧹 Limpando containers e volumes...${NC}"
    docker-compose down -v
    docker system prune -f
    ;;
  "restart")
    echo -e "${YELLOW}🔄 Reiniciando serviços...${NC}"
    docker-compose down
    docker-compose up --build -d
    ;;
  "demo")
    echo -e "${GREEN}🎬 Executando demonstração completa...${NC}"
    echo -e "${BLUE}1. Iniciando serviços...${NC}"
    docker-compose up --build -d
    
    echo -e "${BLUE}2. Aguardando inicialização (15s)...${NC}"
    sleep 15
    
    echo -e "${BLUE}3. Testando endpoint de criação...${NC}"
    curl -X POST http://localhost:8082/programadores \
      -H "Content-Type: application/json" \
      -d '{
        "apelido": "demo123",
        "nome": "Demo da Silva",
        "nascimento": "1990-01-01",
        "stack": ["Rust", "PostgreSQL", "Docker"]
      }' | echo ""
    
    echo -e "${BLUE}4. Consultando contagem...${NC}"
    curl -s http://localhost:8082/contagem-programadores
    echo ""
    
    echo -e "${BLUE}5. Executando teste de stress...${NC}"
    docker-compose run --rm k6 k6 run /tests/desafio_interno_k6.js
    
    echo -e "${BLUE}6. Contagem final...${NC}"
    curl -s http://localhost:8082/contagem-programadores
    echo ""
    ;;
  *)
    echo -e "${BLUE}Comandos disponíveis:${NC}"
    echo ""
    echo -e "${GREEN}  build${NC}        - Constrói as imagens Docker"
    echo -e "${GREEN}  up/start${NC}     - Inicia todos os serviços"
    echo -e "${GREEN}  down/stop${NC}    - Para todos os serviços"
    echo -e "${GREEN}  test${NC}         - Executa teste de stress (container)"
    echo -e "${GREEN}  test-local${NC}   - Executa teste local (k6 local)"
    echo -e "${GREEN}  count${NC}        - Consulta contagem de programadores"
    echo -e "${GREEN}  logs${NC}         - Mostra logs em tempo real"
    echo -e "${GREEN}  status${NC}       - Mostra status dos containers"
    echo -e "${GREEN}  stats${NC}        - Mostra estatísticas de recursos"
    echo -e "${GREEN}  restart${NC}      - Reinicia todos os serviços"
    echo -e "${GREEN}  clean${NC}        - Remove containers e volumes"
    echo -e "${GREEN}  demo${NC}         - Executa demonstração completa"
    echo ""
    echo -e "${YELLOW}Exemplos:${NC}"
    echo -e "  ${BLUE}./run.sh up${NC}      # Inicia os serviços"
    echo -e "  ${BLUE}./run.sh test${NC}    # Executa teste de stress"
    echo -e "  ${BLUE}./run.sh demo${NC}    # Demonstração completa"
    ;;
esac 