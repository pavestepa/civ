export type WireMessage =
  | { kind: "req"; op: string; id: number; body: unknown }
  | { kind: "res"; op: string; id: number; body: unknown }
  | { kind: "evt"; op: string; body: unknown };
