enum _FieldType {
    OneLine, Text, Number
}
type FieldType = {
    name: _FieldType,
    regex?: RegExp
}

enum Attributes {
    NotRoot
}

class Category {

    name: string
    fields: FieldType[]
    attributes: Attributes[]

    constructor() { 
    }
}

class Bond {

}

class BoardRules {

    categorys: Map<string, Category>

    constructor() {
    }

    create_category(name: string) {

    }

    create_bond(from: string, to: string): boolean {
        return true;
    }

}