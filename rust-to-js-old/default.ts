export class Default {
    constructor(readonly fields: any[]) {}
    field(index: number) {
        if (index >= this.fields.length) {
            throw new Error(`index is out of bounds: ${index}`);
        }
        return this.fields[index];
    }
}

export class Address {
    constructor(readonly address: number) {}
}
