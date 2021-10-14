export default function pickRandomItem<T>(list: Readonly<T[]>) {
  return list[Math.floor(list.length * Math.random())];
}
