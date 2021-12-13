// TODO. You need it for ARM device. FUCKKKKKKKKKKKK.

// // https://stackoverflow.com/a/19606031
// function checkLittleEndian(): boolean {
//   var arrayBuffer = new ArrayBuffer(2);
//   var uint8Array = new Uint8Array(arrayBuffer);
//   var uint16array = new Uint16Array(arrayBuffer);
//   uint8Array[0] = 0xaa; // set first byte
//   uint8Array[1] = 0xbb; // set second byte
//   if (uint16array[0] === 0xbbaa) {
//     return true;
//   }
//   if (uint16array[0] === 0xaabb) {
//     return false;
//   }
//   throw new Error("Something crazy just happened");
// }
// const isLittleEndian = checkLittleEndian();

// export namespace LittleEndian {
//   function uint8(elements: Iterable<number>): Uint8Array {
//     return new Uint8Array(elements);
//   }
//   function uint16(elements: Iterable<number>): Uint16Array {
//     const array = new Uint16Array(elements);
//     if (isLittleEndian) {
//       return array;
//     }
//     const dataView = new DataView(array.buffer);
//     let index = 0;
//     for (const element of elements) {
//       dataView.setUint16(index, element, true);
//       index += 2;
//     }
//     return array;
//   }
// }
