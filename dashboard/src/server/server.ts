import express from 'express';
import { handler } from '../../build/handler.js';
import { createKafka } from './kafkaHandle.ts';

const app = express();

app.get('/healthcheck', (req, res) => {
  res.end('ok');
});

var clientId = 'dashboard'
var brokers = ['kafka:9093']
createKafka(clientId, brokers);

app.use(handler);

app.listen(3000, () => {
  console.log('listening on port 3000');
});