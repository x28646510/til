export function print(buf: Uint8Array) {
  Deno.stdout.write(buf);
}
