(async () => {
  const { body } = await fetch("https://deno.land/");
  Deno.copy(Deno.stdout, body);
})();
