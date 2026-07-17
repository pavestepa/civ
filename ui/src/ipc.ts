import { invoke } from "@tauri-apps/api/core";

export type UiAction =
  | { action: "EndTurn" }
  | { action: "AddGold"; data: { player: number; amount: number } }
  | {
      action: "SpawnUnit";
      data: { owner: number; q: number; r: number; kind: string };
    }
  | {
      action: "CreateCity";
      data: { owner: number; q: number; r: number; name: string };
    }
  | { action: "RequestSnapshot" };

export async function dispatchCommand(action: UiAction): Promise<number> {
  return invoke<number>("dispatch_command", { action });
}

export async function pollEvents(): Promise<string[]> {
  return invoke<string[]>("poll_events");
}

export async function launchEngine(): Promise<string> {
  return invoke<string>("launch_engine");
}
