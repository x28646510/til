import { print } from './util.ts';

(async () => {
  const file = await Deno.open("resources/foo.txt");
  const buf = new Uint8Array(100);
  await file.read(buf);
  file.close();
  print(buf);
})();
