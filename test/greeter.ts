#!/usr/bin/env -S deno run --allow-net="localhost" --unsafely-ignore-certificate-errors="localhost"

// run server: cargo run
// run client: deno run -A --unsafely-ignore-certificate-errors .\test\greeter.ts

import { HttpTransport } from "../target/codegen/http.transport.ts";
import Greeter from "../target/codegen/Greeter.ts";

let greeter = new Greeter(new HttpTransport("https://localhost:4433/greeter"));
let reply = await greeter.SayHello({ name: "Nur" });

console.log(reply);
