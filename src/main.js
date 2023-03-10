const { invoke } = window.__TAURI__.tauri;

// TODO: handle invalid `offset` and invalid `len`
function decodeBuf(bytes, offset) {
  let len = bytes[offset[0]];
  let newOffset = offset[0] + 1 + len;
  let buf = new Uint8Array(bytes.slice(offset[0] + 1, newOffset));
  offset[0] = newOffset;
  return buf;
}

function decodeString(bytes, offset) {
  return new TextDecoder("utf-8").decode(decodeBuf(bytes, offset));
}

function decodeU128(bytes, offset) {
  return BigInt(decodeString(bytes, offset));
}

// TODO: Throwing all this directly on the `window` is probs bad. Put it into a namespace.
window.decodeBuf = decodeBuf;
window.decodeString = decodeString;
window.decodeU128 = decodeU128;

// TODO: This is an example of what is inject by the Tauri plugin
// const DECODE_TABLE = {
//   my_func_wrapper: (data) => {
//     let offset = [0]; // Array is so JS is pass by value not pass by reference - Why can't JS just be Rust lmao
//     const field1 = decodeString(data, offset);
//     console.log(offset);
//     const field2 = decodeBuf(data, offset);

//     return {
//       name: field1,
//       data: field2,
//     };
//   },
// };

async function invoke2(key) {
  return window.INJECTED_DECODE_TABLE[key](await invoke(key));
}

(async () => {
  const data = await invoke("my_func_wrapper"); // TODO: Probs convert to `ArrayBuffer` or `Int8Array` or something. Idk much about them.
  console.log(data);

  const data2 = await invoke2("my_func_wrapper");
  console.log(data2);
})();
