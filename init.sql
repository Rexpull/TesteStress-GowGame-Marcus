-- Configurações de otimização para PostgreSQL
-- Este arquivo será executado na inicialização do banco

-- Criar extensão para UUID se não existir
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Configurar algumas opções de performance (se possível)
-- Nota: Algumas configurações precisam ser definidas no postgresql.conf
-- mas estas podem ser aplicadas por sessão

-- Configurar work_mem para esta sessão
SET work_mem = '4MB';

-- Configurar random_page_cost para SSD
SET random_page_cost = 1.1;

-- Log de inicialização
DO $$
BEGIN
    RAISE NOTICE 'Banco de dados inicializado com sucesso para o desafio GOW!';
END
$$; 