import { Var } from "./impl";

export function _add(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue + rhsValue);
}
export function _sub(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue - rhsValue);
}
export function _mul(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue * rhsValue);
}
export function _div(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue / rhsValue);
}
export function _rem(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue % rhsValue);
}
export function _xor(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue ^ rhsValue);
}
export function _and(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue & rhsValue);
}
export function _or(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue | rhsValue);
}
export function _shl(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue << rhsValue);
}
export function _shr(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue >> rhsValue);
}
export function _eq(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue === rhsValue);
}
export function _lt(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue < rhsValue);
}
export function _le(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue <= rhsValue);
}
export function _ne(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue !== rhsValue);
}
export function _ge(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue >= rhsValue);
}
export function _gt(lhs: any, rhs: any) {
    let lhsValue = lhs instanceof Var ? lhs.value : lhs;
    let rhsValue = rhs instanceof Var ? rhs.value : rhs;

    return new Var(lhsValue > rhsValue);
}
export function _not(arg: any) {
    let argValue = arg instanceof Var ? arg.value : arg;

    return new Var(!argValue);
}
