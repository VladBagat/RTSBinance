import { createConsumer } from "../server/kafkaHandle";

var groupId = '0'
var consumer = createConsumer(groupId)

var topic = 'binance-liquidations-processed'

await consumer?.connect()
await consumer?.subscribe({ topic, fromBeginning: true })


export const load = async () => {
	await consumer?.run({
		eachMessage: async ({ topic, partition, message }) => {
		const prefix = `${topic}[${partition} | ${message.offset}] / ${message.timestamp}`
		console.log(`- ${prefix} ${message.key}#${message.value}`)
		},
	})
};