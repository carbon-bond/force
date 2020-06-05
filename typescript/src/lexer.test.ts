import { lexer } from './lexer';

test('lexer 解析特殊符號', () => {
	lexer.reset('{}[],#:');
	let token = lexer.next();
	expect(token!.type!).toBe('left_curly_brace');
	token = lexer.next();
	expect(token!.type!).toBe('right_curly_brace');
	token = lexer.next();
	expect(token!.type!).toBe('left_square_bracket');
	token = lexer.next();
	expect(token!.type!).toBe('right_square_bracket');
	token = lexer.next();
	expect(token!.type!).toBe('comma');
	token = lexer.next();
	expect(token!.type!).toBe('sharp');
	token = lexer.next();
	expect(token!.type!).toBe('colon');
	token = lexer.next();
	expect(token).toBe(undefined);
});

test('lexer 解析關鍵字', () => {
	lexer.reset('單行文本數字鍵結帶籤鍵結輸能');
	let token = lexer.next();
	expect(token!.type!).toBe('one_line');
	token = lexer.next();
	expect(token!.type!).toBe('text');
	token = lexer.next();
	expect(token!.type!).toBe('number');
	token = lexer.next();
	expect(token!.type!).toBe('bond');
	token = lexer.next();
	expect(token!.type!).toBe('tagged_bond');
	token = lexer.next();
	expect(token!.type!).toBe('transfuse');
	token = lexer.next();
	expect(token).toBe(undefined);
});

test('lexer 解析識別子', () => {
	lexer.reset('Gossip');
	let token = lexer.next();
	expect(token!.type!).toBe('identifier');
	lexer.reset('八卦');
	token = lexer.next();
	expect(token!.type!).toBe('identifier');
	lexer.reset('play_boy');
	token = lexer.next();
	expect(token!.type!).toBe('identifier');
	lexer.reset('花花公子');
	token = lexer.next();
	expect(token!.type!).toBe('identifier');
});

test('lexer 解析正則表達式', () => {
	lexer.reset('/[ab]+d?/');
	let token = lexer.next();
	expect(token!.type!).toBe('regex');
});