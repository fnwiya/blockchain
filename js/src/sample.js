/* @flow */
import {
    getGenesisBlock,
    generateNextBlock,
    getLatestBlock,
    addBlock,
    isValidChain,
    isValidNewBlock
} from './index'

const blockchain = [getGenesisBlock()]

const prev = getLatestBlock(blockchain)
const next1 = generateNextBlock(blockchain, 'foo')
const newBlockchain1 = addBlock(blockchain, next1)

const next2 = generateNextBlock(newBlockchain1, 'bar')
const newBlockchain2 = addBlock(newBlockchain1, next2)

console.log('current', newBlockchain2)
console.log('isValidChain', isValidChain(newBlockchain2))
