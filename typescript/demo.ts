import * as parser from './src/parser';
import { lexer } from './src/lexer';

const tokens = lexer('{}{}[[]]');

console.log(tokens);
