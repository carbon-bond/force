import { Lexer, createLexer, IToken } from './lexer_base';

export function lexer(s: string): IToken[] {
    return createLexer([
        {
            type: 'whitespace',
            regexes: [/^(\s+)/],
            ignore: true
        },
        {
            type: '{',
            regexes: [/({)/],
        },
        {
            type: '}',
            regexes: [/(})/],
        },
        {
            type: '[',
            regexes: [/(\[)/],
        },
        {
            type: '[',
            regexes: [/(\])/],
        },
    ])(s);
}