import http from 'k6/http';
import { check } from 'k6';
import { uuidv4 } from 'https://jslib.k6.io/k6-utils/1.4.0/index.js';

export let options = {
  vus: 50,
  duration: '5s',
};

export default function () {
  const payload = JSON.stringify({
    apelido: `dev-${uuidv4().substring(0, 8)}`,
    nome: `Pessoa ${uuidv4()}`,
    nascimento: `${1980 + Math.floor(Math.random() * 25)}-${String(Math.floor(Math.random() * 12) + 1).padStart(2, '0')}-${String(Math.floor(Math.random() * 28) + 1).padStart(2, '0')}`,
    stack: ["Rust", "PostgreSQL", "Docker", "k6"].slice(0, Math.floor(Math.random() * 4) + 1)
  });

  const headers = { 'Content-Type': 'application/json' };
  const res = http.post('http://localhost:9999/programadores', payload, { headers });

  check(res, {
    'status é 201 (Created)': (r) => r.status === 201,
  });
}

export function testCount() {
  const res = http.get('http://localhost:9999/contagem-programadores');
  
  check(res, {
    'contagem status 200': (r) => r.status === 200,
    'resposta é número': (r) => !isNaN(parseInt(r.body)),
  });
  
  console.log(`Total de programadores: ${res.body}`);
} 