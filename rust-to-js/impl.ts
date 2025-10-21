import { Address, Default } from "./default";

export {
    AllocatedMemory,
    alloc__alloc__exchange_malloc,
    std__ptr__NonNull,
    std__ptr__Unique,
    StackMemory,
} from "./memory";
export {
    _add,
    _sub,
    _mul,
    _div,
    _rem,
    _xor,
    _and,
    _or,
    _shl,
    _shr,
    _eq,
    _lt,
    _le,
    _ne,
    _ge,
    _gt,
    _not,
} from "./ops";

export function core__fmt__rt__Argument__new_display(arg) {
    return arg.toString();
}
export function core__fmt__rt__Arguments__new_v1(format_strings, args) {
    return { format_strings, args };
}
export function core__fmt__rt__Arguments__new_const(format_strings) {
    return { format_strings, args: [] };
}
export function std__io__print(args) {
    let output = "";
    for (let i = 0; i < args.format_strings.length; i++) {
        output += args.format_strings[i];
        if (i < args.args.length) {
            output += args.args[i];
        }
    }
    console.log(output);
}
export function std__iter__IntoIterator__into_iter(arg: Var) {
    return arg.value.into_iter();
}
export function std__iter__Iterator__next(arg: Var) {
    return arg.value.next();
}
export function discriminant(arg) {
    if (!(arg instanceof Enum)) {
        throw new Error("arg is not enum");
    }
    return arg.discriminant;
}
export function switchInt(arg: Var) {
    const { value } = arg;
    if (typeof value === "number") {
        return value;
    }
    throw new Error("not implemented");
}

export function std__slice__impl__T__into_vec(slice) {
    return new Vec(slice);
}

export function _ref(arg: any) {
    if (arg instanceof NoRefVar) {
        return arg.ptr;
    }
    console.log("ref arg", arg);
    throw new Error("not implemented");
}

export class Var {
    value: any;
    constructor(value: any) {
        this.value = value;
    }
    assign(arg) {
        this.value = arg instanceof Var ? arg.value : arg;
    }
    field(index: number) {
        return this.value.field(index);
    }
    deref() {
        return this.value.deref();
    }
}

export class Vec {
    constructor(slice) {
        this.len = slice.length;
        this.values = slice;
    }
    intoIter() {
        let i = 0;
        return {
            next() {
                if (i < this.len) {
                    return new Enum(this.values[i++], 1);
                }
                return new Enum(undefined, 0);
            },
        };
    }
}

export class Array {
    constructor(readonly slice: readonly any[]) {}
    into_iter() {
        const { slice } = this;
        let i = 0;
        return {
            next() {
                if (i < slice.length) {
                    return new Enum(slice[i++], 1);
                }
                return new Enum(undefined, 0);
            },
        };
    }
    _to_bytes() {
        console.log("this", this);
        const bytesList = this.slice.map((item) => {
            return _to_bytes(item);
        });
        const totalBytes = bytesList.reduce((acc, bytes) => {
            return acc + bytes.length;
        }, 0);
        const result = new Uint8Array(totalBytes);
        let offset = 0;
        bytesList.forEach((bytes) => {
            result.set(bytes, offset);
            offset += bytes.length;
        });
        return result;
    }
}

export class Tuple extends Default {}

export class Enum extends Default {
    constructor(value: any, private readonly discriminant: number) {
        super([value]);
    }
}

enum VarType {
    NoRef = 0,
    Ref = 1,
    MutRef = 2,
}

export class LocalVar {
    constructor(
        private readonly memory: Uint8Array,
        private readonly varType: VarType,
    ) {}

    assign(value: any) {
        switch (this.varType) {
            case VarType.NoRef: {
                this.memory.set(_to_bytes(value));
            }
            case VarType.Ref: {
                console.log(value);
                throw new Error("not implemented");
            }
            case VarType.MutRef: {
                console.log(value);
                throw new Error("not implemented");
            }
        }
    }
}

export class NoRefVar {
    constructor(readonly ptr: number, readonly size: number) {}
    assign(value) {
        new Uint8Array(memory.buffer, this.ptr, this.size).set(
            _to_bytes(value),
        );
    }
}

export class RefVar {
    constructor(private readonly ptr: number, private readonly size: number) {}
    assign(value: number) {
        new Uint8Array(memory.buffer, this.ptr, this.size).set([
            value & 0xff,
            (value >> 8) & 0xff,
            (value >> 16) & 0xff,
            (value >> 24) & 0xff,
            (value >> 32) & 0xff,
            (value >> 40) & 0xff,
            (value >> 48) & 0xff,
            (value >> 56) & 0xff,
        ]);
    }
}

export class MutRefVar {
    constructor(private readonly ptr: number, private readonly size: number) {}
}

export const memory = {
    buffer: new Uint8Array(1024 * 1024),
    nextPtr: 0,
    stackAlloc(size: number): number {
        const ptr = this.nextPtr;
        this.nextPtr += size;
        return ptr;
    },
};

function _to_bytes(arg): Uint8Array {
    if (arg instanceof Uint8Array) {
        return arg;
    }
    console.log(arg);
    if ("_to_bytes" in arg) {
        return arg._to_bytes();
    }
    throw new Error("not implemented");
}

export class Int8 {
    constructor(readonly value: number) {}
    _to_bytes() {
        return new Uint8Array([this.value]);
    }
}

export class Int16 {
    constructor(readonly value: number) {}
    _to_bytes() {
        return new Uint8Array([this.value & 0xff, (this.value >> 8) & 0xff]);
    }
}

export class Int32 {
    constructor(readonly value: number) {}
    _to_bytes() {
        return new Uint8Array([
            this.value & 0xff,
            (this.value >> 8) & 0xff,
            (this.value >> 16) & 0xff,
            (this.value >> 24) & 0xff,
        ]);
    }
}

export class Int64 {
    constructor(readonly value: bigint) {}
    _to_bytes() {
        return new Uint8Array([
            Number(this.value & 0xffn),
            Number((this.value >> 8n) & 0xffn),
            Number((this.value >> 16n) & 0xffn),
            Number((this.value >> 24n) & 0xffn),
            Number((this.value >> 32n) & 0xffn),
            Number((this.value >> 40n) & 0xffn),
            Number((this.value >> 48n) & 0xffn),
            Number((this.value >> 56n) & 0xffn),
        ]);
    }
}

export class Uint8 {
    constructor(readonly value: number) {}
    _to_bytes() {
        return new Uint8Array([this.value]);
    }
}

export class Uint16 {
    constructor(readonly value: number) {}
    _to_bytes() {
        return new Uint8Array([this.value & 0xff, (this.value >> 8) & 0xff]);
    }
}

export class Uint32 {
    constructor(readonly value: number) {}
    _to_bytes() {
        return new Uint8Array([
            this.value & 0xff,
            (this.value >> 8) & 0xff,
            (this.value >> 16) & 0xff,
            (this.value >> 24) & 0xff,
        ]);
    }
}

export class Uint64 {
    constructor(readonly value: bigint) {}
    _to_bytes() {
        return new Uint8Array([
            Number(this.value & 0xffn),
            Number((this.value >> 8n) & 0xffn),
            Number((this.value >> 16n) & 0xffn),
            Number((this.value >> 24n) & 0xffn),
            Number((this.value >> 32n) & 0xffn),
            Number((this.value >> 40n) & 0xffn),
            Number((this.value >> 48n) & 0xffn),
            Number((this.value >> 56n) & 0xffn),
        ]);
    }
}
