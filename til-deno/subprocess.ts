const hello = async () => {
  const p = Deno.run({
    args: ["echo", "hello"]
  })
  await p.status();
}

hello();

const hello_pipe = async () => {
  const p = Deno.run({
    args: ["echo", "hello"],
    stdout: "piped"
  })

  const rawOutput = await p.output();
  Deno.stdout.write(rawOutput);
}

hello_pipe();
