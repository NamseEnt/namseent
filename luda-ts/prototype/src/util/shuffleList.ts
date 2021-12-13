export default function shuffleList<T>(list: Readonly<T[]>): T[] {
  return list
    .map((item) => [Math.random(), item] as [number, T])
    .sort((a, b) => b[0] - a[0])
    .map((item) => item[1]);
}
