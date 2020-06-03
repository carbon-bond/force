import * as moo from 'moo';

let lexer = moo.compile({
	whitespace: /[ \t]+/,
	new_line: { match: /\n/, lineBreaks: true },

	// 特殊符號
	left_curly_brace: '{',
	right_curly_brace: '}',
	left_square_bracket: '[',
	right_square_bracket: ']',
	comma: ',',
	sharp: '#',
	colon: ':',

	one_line: '單行',
	text: '文本',
	number: '數字',
	bond: '鍵結',
	tagged_bond: '帶籤鍵結',

	transufse: '輸能',
	star: '*',

	regex: new RegExp('/[^/]+/'),
	identifier: /[^\s/\[\]]+/
});

export {
	lexer
};