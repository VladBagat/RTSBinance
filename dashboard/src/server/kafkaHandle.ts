import { type Consumer, Kafka } from "kafkajs";

let kafka: Kafka | null = null

let consumer: Consumer | null = null

export function createKafka(clientId:string, brokers: string[]) {
    kafka = new Kafka({
        clientId,
        brokers,
    })
    return kafka
}

export function createConsumer(groupId: string) {
    consumer = kafka === null ? null : kafka.consumer({ groupId })
    return consumer
}

export function getConsumer() {
    return consumer
}